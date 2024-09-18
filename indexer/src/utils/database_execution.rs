//! Database-related functions
#![allow(clippy::extra_unused_lifetimes)]

use diesel::{query_builder::QueryFragment, QueryResult};
use diesel_async::RunQueryDsl;
use tracing::{debug, warn};

use super::database_utils::{clean_data_for_db, ArcDbPool, Backend, MyDbConnection};
use crate::utils::database_utils::UpsertFilterLatestTransactionQuery;

pub async fn execute_in_chunks<U, T>(
    pool: ArcDbPool,
    build_query: fn(Vec<T>) -> (U, Option<&'static str>),
    items_to_insert: &[T],
    chunk_size: usize,
) -> Result<(), diesel::result::Error>
where
    U: QueryFragment<Backend> + diesel::query_builder::QueryId + Send + 'static,
    T: serde::Serialize + for<'de> serde::Deserialize<'de> + Clone + Send + 'static,
{
    let tasks = items_to_insert
        .chunks(chunk_size)
        .map(|chunk| {
            let pool = pool.clone();
            let items = chunk.to_vec();
            tokio::spawn(async move {
                let (query, additional_where_clause) = build_query(items.clone());
                execute_or_retry_cleaned(pool, build_query, items, query, additional_where_clause)
                    .await
            })
        })
        .collect::<Vec<_>>();

    let results = futures_util::future::try_join_all(tasks)
        .await
        .expect("Task panicked executing in chunks");
    for res in results {
        res?
    }

    Ok(())
}

pub async fn execute_or_retry_cleaned<U, T>(
    pool: ArcDbPool,
    build_query: fn(Vec<T>) -> (U, Option<&'static str>),
    items: Vec<T>,
    query: U,
    additional_where_clause: Option<&'static str>,
) -> Result<(), diesel::result::Error>
where
    U: QueryFragment<Backend> + diesel::query_builder::QueryId + Send,
    T: serde::Serialize + for<'de> serde::Deserialize<'de> + Clone,
{
    match execute_with_better_error(pool.clone(), query, additional_where_clause).await {
        Ok(_) => {}
        Err(_) => {
            let cleaned_items = clean_data_for_db(items, true);
            let (cleaned_query, additional_where_clause) = build_query(cleaned_items);
            match execute_with_better_error(pool.clone(), cleaned_query, additional_where_clause)
                .await
            {
                Ok(_) => {}
                Err(e) => {
                    return Err(e);
                }
            }
        }
    }
    Ok(())
}

pub async fn execute_with_better_error<U>(
    pool: ArcDbPool,
    query: U,
    additional_where_clause: Option<&'static str>,
) -> QueryResult<usize>
where
    U: QueryFragment<Backend> + diesel::query_builder::QueryId + Send,
{
    let conn = &mut pool.get().await.map_err(|e| {
        warn!("Error getting connection from pool: {:?}", e);
        diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UnableToSendCommand,
            Box::new(e.to_string()),
        )
    })?;

    execute_with_better_error_conn(conn, query, additional_where_clause).await
}

pub async fn execute_with_better_error_conn<U>(
    conn: &mut MyDbConnection,
    query: U,
    mut additional_where_clause: Option<&'static str>,
) -> QueryResult<usize>
where
    U: QueryFragment<Backend> + diesel::query_builder::QueryId + Send,
{
    let original_query = diesel::debug_query::<Backend, _>(&query).to_string();
    // This is needed because if we don't insert any row, then diesel makes a call like this
    // SELECT 1 FROM TABLE WHERE 1=0
    if original_query.to_lowercase().contains("where") {
        additional_where_clause = None;
    }
    let final_query = UpsertFilterLatestTransactionQuery {
        query,
        where_clause: additional_where_clause,
    };
    let debug_string = diesel::debug_query::<Backend, _>(&final_query).to_string();
    debug!("Executing query: {:?}", debug_string);
    let res = final_query.execute(conn).await;
    if let Err(ref e) = res {
        warn!("Error running query: {:?}\n{:?}", e, debug_string);
    }
    res
}
