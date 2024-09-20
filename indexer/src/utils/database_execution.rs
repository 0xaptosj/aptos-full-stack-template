use diesel::{
    debug_query,
    query_builder::{QueryFragment, QueryId},
    QueryResult,
};
use diesel_async::{AsyncConnection, RunQueryDsl};
use tracing::{debug, warn};

use super::database_utils::{Backend, MyDbConnection};

pub async fn execute_with_better_error<U>(
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
