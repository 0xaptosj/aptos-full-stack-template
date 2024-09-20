use diesel::{
    debug_query,
    pg::Pg,
    query_builder::{QueryFragment, QueryId},
    result::{DatabaseErrorKind, Error as DieselError},
    QueryResult,
};
use diesel_async::{AsyncConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};
use tracing::{debug, warn};

use super::database_utils::{clean_data_for_db, ArcDbPool, Backend, MyDbConnection};
use crate::{
    db_models::events_models::Message,
    processors::events::{
        events_storer::EventStorerAction,
        storers::{
            create_message_event_storer::create_message_events_sql,
            update_message_event_storer::update_message_events_sql,
        },
    },
};

pub async fn execute_in_chunks<T>(
    pool: ArcDbPool,
    action: EventStorerAction,
    items_to_insert: &[T],
    chunk_size: usize,
) -> Result<(), DieselError>
where
    T: Serialize + for<'de> Deserialize<'de> + Clone + Send + 'static,
{
    let tasks = items_to_insert
        .chunks(chunk_size)
        .map(|chunk| {
            let pool = pool.clone();
            let items = chunk.to_vec();
            let action = action.clone();
            tokio::spawn(async move { execute_or_retry_cleaned(pool, &action, items).await })
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

pub async fn execute_or_retry_cleaned<T>(
    pool: ArcDbPool,
    action: &EventStorerAction,
    items: Vec<T>,
) -> Result<(), DieselError>
where
    T: Serialize + for<'de> Deserialize<'de> + Clone,
{
    let cloned_pool_1 = pool.clone();
    let conn_1: &mut MyDbConnection = &mut cloned_pool_1
        .get()
        .await
        .map_err(|e| {
            warn!("Error getting connection from pool: {:?}", e);
            DieselError::DatabaseError(
                DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string()),
            )
        })
        .unwrap();
    match build_query_and_execute_with_better_error_conn(conn_1, action, items.clone()).await {
        Ok(_) => {}
        Err(_) => {
            let cloned_pool_2 = pool.clone();
            let conn_2 = &mut cloned_pool_2.get().await.map_err(|e| {
                warn!("Error getting connection from pool: {:?}", e);
                DieselError::DatabaseError(
                    DatabaseErrorKind::UnableToSendCommand,
                    Box::new(e.to_string()),
                )
            })?;
            let cleaned_items = clean_data_for_db(items, true);
            match build_query_and_execute_with_better_error_conn(conn_2, action, cleaned_items)
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

pub async fn build_query_and_execute_with_better_error_conn<T, U>(
    conn: &mut MyDbConnection,
    action: &EventStorerAction,
    items: Vec<T>,
) -> QueryResult<()>
where
    T: Serialize + for<'de> Deserialize<'de> + Clone,
    U: QueryFragment<Backend> + QueryId + Send,
{
    let queries = match action {
        EventStorerAction::CreateMessage => create_message_events_sql(conn, items).await,
        EventStorerAction::UpdateMessage => update_message_events_sql(conn, items).await,
    };
    execute_with_better_error_conn(conn, queries).await
}

pub async fn execute_with_better_error<U>(pool: ArcDbPool, queries: Vec<U>) -> QueryResult<()>
where
    U: QueryFragment<Backend> + QueryId + Send,
{
    let conn = &mut pool.get().await.map_err(|e| {
        warn!("Error getting connection from pool: {:?}", e);
        DieselError::DatabaseError(
            DatabaseErrorKind::UnableToSendCommand,
            Box::new(e.to_string()),
        )
    })?;

    execute_with_better_error_conn(conn, queries).await
}

pub async fn execute_with_better_error_conn<U>(
    conn: &mut MyDbConnection,
    queries: Vec<U>,
) -> QueryResult<()>
where
    U: QueryFragment<Backend> + QueryId + Send,
{
    let debug_query = queries
        .iter()
        .map(|q| debug_query::<Backend, _>(q).to_string())
        .collect::<Vec<_>>();
    debug!(
        "Executing queries in one DB transaction atomically: {:?}",
        debug_query
    );
    let res = conn
        .transaction(|conn| {
            Box::pin(async move {
                for q in queries {
                    q.execute(conn).await?;
                }
                Ok(())
            })
        })
        .await;
    if let Err(ref e) = res {
        warn!("Error running query: {:?}\n{:?}", e, debug_query);
    }
    res
}
