use ahash::AHashMap;
use anyhow::Result;
use aptos_indexer_processor_sdk::utils::errors::ProcessorError;
use diesel::{insert_into, upsert::excluded, ExpressionMethods, QueryResult};
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};

use crate::{
    db_models::{message::Message, user_stat::UserStat},
    schema::{messages, user_stats},
    utils::{
        database_connection::get_db_connection,
        database_utils::{get_config_table_chunk_size, ArcDbPool},
        time::current_unix_timestamp_in_seconds,
    },
};

const POINT_PER_NEW_MESSAGE: i64 = 2;

async fn execute_create_message_events_sql(
    conn: &mut AsyncPgConnection,
    items_to_insert: Vec<Message>,
    user_new_message_counts: AHashMap<String, i64>,
) -> QueryResult<()> {
    conn.transaction(|conn| {
        Box::pin(async move {
            let create_message_query = insert_into(messages::table)
                .values(items_to_insert.clone())
                .on_conflict(messages::message_obj_addr)
                .do_nothing();
            create_message_query.execute(conn).await?;

            /*
            DO NOT backfill data (i.e. process same event twice), you would mess up the user stat!!!!
            Instead, if you want to change the point calculation logic, you should delete all data and re-index from scratch.
            You can delete all data by revert all DB migrations, see README.md for more details.
             */
            let update_user_stat_query = insert_into(user_stats::table)
                .values(
                    user_new_message_counts
                        .iter()
                        .map(|(user_addr, new_message_count)| UserStat {
                            user_addr: user_addr.clone(),
                            creation_timestamp: current_unix_timestamp_in_seconds(),
                            last_update_timestamp: current_unix_timestamp_in_seconds(),
                            created_messages: *new_message_count,
                            updated_messages: 0,
                            s1_points: new_message_count * POINT_PER_NEW_MESSAGE,
                            total_points: new_message_count * POINT_PER_NEW_MESSAGE,
                        })
                        .collect::<Vec<_>>(),
                )
                .on_conflict(user_stats::user_addr)
                .do_update()
                .set((
                    user_stats::user_addr.eq(user_stats::user_addr),
                    user_stats::creation_timestamp.eq(user_stats::creation_timestamp),
                    user_stats::last_update_timestamp
                        .eq(excluded(user_stats::last_update_timestamp)),
                    user_stats::created_messages
                        .eq(user_stats::created_messages + excluded(user_stats::created_messages)),
                    user_stats::updated_messages.eq(user_stats::updated_messages),
                    user_stats::s1_points
                        .eq(user_stats::s1_points + excluded(user_stats::s1_points)),
                    user_stats::total_points
                        .eq(user_stats::total_points + excluded(user_stats::total_points)),
                ));
            update_user_stat_query.execute(conn).await?;

            Ok(())
        })
    })
    .await
}

pub async fn process_create_message_events(
    pool: ArcDbPool,
    per_table_chunk_sizes: AHashMap<String, usize>,
    create_events: Vec<Message>,
) -> Result<(), ProcessorError> {
    let mut user_new_message_counts: AHashMap<String, i64> = AHashMap::new();
    for message in create_events.clone() {
        let new_count = user_new_message_counts
            .get(&message.creator_addr)
            .unwrap_or(&0)
            + 1;
        user_new_message_counts.insert(message.creator_addr.clone(), new_count);
    }

    let chunk_size = get_config_table_chunk_size::<Message>("messages", &per_table_chunk_sizes);
    let tasks = create_events
        .chunks(chunk_size)
        .map(|chunk| {
            let pool = pool.clone();
            let items = chunk.to_vec();
            let user_new_message_counts = user_new_message_counts.clone();
            tokio::spawn(async move {
                let conn = &mut get_db_connection(&pool).await.expect(
                    "Failed to get connection from pool while processing create message events",
                );
                execute_create_message_events_sql(conn, items, user_new_message_counts).await
            })
        })
        .collect::<Vec<_>>();

    let results = futures_util::future::try_join_all(tasks)
        .await
        .expect("Task panicked executing in chunks");
    for res in results {
        res.map_err(|e| {
            tracing::warn!("Error running query: {:?}", e);
            ProcessorError::ProcessError {
                message: e.to_string(),
            }
        })?;
    }
    Ok(())
}
