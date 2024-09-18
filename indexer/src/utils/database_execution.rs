//! Database-related functions
#![allow(clippy::extra_unused_lifetimes)]

use diesel::{query_builder::QueryFragment, QueryResult};
use diesel_async::RunQueryDsl;
use tracing::{debug, warn};

use super::database_utils::{clean_data_for_db, ArcDbPool, Backend, MyDbConnection};

pub async fn execute_in_chunks<U, T>(
    pool: ArcDbPool,
    build_query: fn(Vec<T>) -> U,
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
                let query = build_query(items.clone());
                execute_or_retry_cleaned(pool, build_query, items, query).await
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
    build_query: fn(Vec<T>) -> U,
    items: Vec<T>,
    query: U,
) -> Result<(), diesel::result::Error>
where
    U: QueryFragment<Backend> + diesel::query_builder::QueryId + Send,
    T: serde::Serialize + for<'de> serde::Deserialize<'de> + Clone,
{
    match execute_with_better_error(pool.clone(), query).await {
        Ok(_) => {}
        Err(_) => {
            let cleaned_items = clean_data_for_db(items, true);
            let cleaned_query = build_query(cleaned_items);
            match execute_with_better_error(pool.clone(), cleaned_query).await {
                Ok(_) => {}
                Err(e) => {
                    return Err(e);
                }
            }
        }
    }
    Ok(())
}

pub async fn execute_with_better_error<U>(pool: ArcDbPool, query: U) -> QueryResult<usize>
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

    execute_with_better_error_conn(conn, query).await
}

pub async fn execute_with_better_error_conn<U>(
    conn: &mut MyDbConnection,
    query: U,
) -> QueryResult<usize>
where
    U: QueryFragment<Backend> + diesel::query_builder::QueryId + Send,
{
    let debug_query = diesel::debug_query::<Backend, _>(&query).to_string();
    debug!("Executing query: {:?}", debug_query);
    let res = query.execute(conn).await;
    if let Err(ref e) = res {
        warn!("Error running query: {:?}\n{:?}", e, debug_query);
    }
    res
}
