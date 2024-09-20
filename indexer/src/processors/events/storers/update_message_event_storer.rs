use ahash::AHashMap;
use anyhow::Result;
use aptos_indexer_processor_sdk::utils::errors::ProcessorError;
use diesel::{
    query_dsl::methods::FilterDsl,
    result::{self, DatabaseErrorKind},
    upsert::excluded,
    BoolExpressionMethods, ExpressionMethods, QueryResult,
};
use diesel_async::{AsyncConnection, RunQueryDsl};

use crate::{
    db_models::{message::Message, user_stat::UserStat},
    schema::{messages, user_stats},
    utils::database_utils::{get_config_table_chunk_size, ArcDbPool, MyDbConnection},
};

const POINT_PER_UPDATE_MESSAGE: i64 = 1;

async fn execute_update_message_events_sql(
    conn: &mut MyDbConnection,
    items_to_insert: Vec<Message>,
    user_updated_message_counts: AHashMap<String, i64>,
) -> QueryResult<()> {
    conn.transaction(|conn| {
        Box::pin(async move {
            let update_message_query = diesel::insert_into(messages::table)
                .values(items_to_insert.clone())
                .on_conflict(messages::message_obj_addr)
                .do_update()
                .set((
                    messages::message_obj_addr.eq(excluded(messages::message_obj_addr)),
                    messages::creator_addr.eq(excluded(messages::creator_addr)),
                    messages::creation_timestamp.eq(excluded(messages::creation_timestamp)),
                    messages::last_update_timestamp.eq(excluded(messages::last_update_timestamp)),
                    messages::last_update_event_idx.eq(excluded(messages::last_update_event_idx)),
                    messages::content.eq(excluded(messages::content)),
                ))
                .filter(
                    // Update only if the last update timestamp is greater than the existing one
                    // or if the last update timestamp is the same but the event index is greater
                    messages::last_update_timestamp
                        .lt(excluded(messages::last_update_timestamp))
                        .or(messages::last_update_timestamp
                            .eq(excluded(messages::last_update_timestamp))
                            .and(
                                messages::last_update_event_idx
                                    .lt(excluded(messages::last_update_event_idx)),
                            )),
                );
            update_message_query.execute(conn).await?;

            let current_user_stats: AHashMap<String, UserStat> = user_stats::table
                .filter(user_stats::user_addr.eq_any(user_updated_message_counts.keys()))
                .load::<UserStat>(conn)
                .await?
                .iter()
                .map(|user_point| (user_point.user_addr.clone(), user_point.clone()))
                .collect();
            let updated_user_stats = user_updated_message_counts
                .iter()
                .map(
                    |(user_addr, updated_message_count)| match current_user_stats.get(user_addr) {
                        Some(stat) => UserStat {
                            user_addr: user_addr.clone(),
                            creation_timestamp: stat.creation_timestamp,
                            last_update_timestamp: stat.last_update_timestamp,
                            user_point: stat.user_point
                                + updated_message_count * POINT_PER_UPDATE_MESSAGE,
                            created_messages: stat.created_messages,
                            updated_messages: stat.updated_messages + updated_message_count,
                        },
                        None => UserStat {
                            user_addr: user_addr.clone(),
                            creation_timestamp: 0,
                            last_update_timestamp: 0,
                            user_point: updated_message_count * POINT_PER_UPDATE_MESSAGE,
                            created_messages: 0,
                            updated_messages: *updated_message_count,
                        },
                    },
                )
                .collect::<Vec<_>>();
            let update_user_point_query = diesel::insert_into(user_stats::table)
                .values(updated_user_stats)
                .on_conflict(user_stats::user_addr)
                .do_update()
                .set((
                    user_stats::user_addr.eq(excluded(user_stats::user_addr)),
                    user_stats::creation_timestamp.eq(excluded(user_stats::creation_timestamp)),
                    user_stats::last_update_timestamp
                        .eq(excluded(user_stats::last_update_timestamp)),
                    user_stats::user_point.eq(excluded(user_stats::user_point)),
                    user_stats::created_messages.eq(excluded(user_stats::created_messages)),
                    user_stats::updated_messages.eq(excluded(user_stats::updated_messages)),
                ));
            update_user_point_query.execute(conn).await?;

            Ok(())
        })
    })
    .await
}

pub async fn process_update_message_events(
    pool: ArcDbPool,
    per_table_chunk_sizes: AHashMap<String, usize>,
    update_events: Vec<Message>,
) -> Result<(), ProcessorError> {
    let mut user_updated_message_counts: AHashMap<String, i64> = AHashMap::new();
    for message in update_events.clone() {
        let new_count = user_updated_message_counts
            .get(&message.creator_addr)
            .unwrap_or(&0)
            + 1;
        user_updated_message_counts.insert(message.creator_addr.clone(), new_count);
    }

    // filter update_events so when there are 2 events updating the same record, only the latest one is sent to DB for update
    // because we cannot update one record with 2 different values in the same transaction
    let mut filtered_update_events_map: AHashMap<String, Message> = AHashMap::new();
    for message in update_events {
        filtered_update_events_map
            .entry(message.message_obj_addr.clone())
            .and_modify(|existing| {
                if (message.last_update_timestamp, message.last_update_event_idx)
                    > (
                        existing.last_update_timestamp,
                        existing.last_update_event_idx,
                    )
                {
                    *existing = message.clone();
                }
            })
            .or_insert(message);
    }
    let filtered_update_events: Vec<Message> = filtered_update_events_map.into_values().collect();

    let chunk_size = get_config_table_chunk_size::<Message>("messages", &per_table_chunk_sizes);
    let tasks = filtered_update_events
        .chunks(chunk_size)
        .map(|chunk| {
            let pool = pool.clone();
            let items = chunk.to_vec();
            let user_updated_message_counts = user_updated_message_counts.clone();
            tokio::spawn(async move {
                let conn: &mut MyDbConnection = &mut pool
                    .get()
                    .await
                    .map_err(|e| {
                        tracing::warn!("Error getting connection from pool: {:?}", e);
                        result::Error::DatabaseError(
                            DatabaseErrorKind::UnableToSendCommand,
                            Box::new(e.to_string()),
                        )
                    })
                    .unwrap();
                execute_update_message_events_sql(conn, items, user_updated_message_counts).await
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