use ahash::AHashMap;
use anyhow::Result;
use aptos_indexer_processor_sdk::utils::errors::ProcessorError;
use diesel::{
    result::{self, DatabaseErrorKind},
    upsert::excluded,
    ExpressionMethods, QueryDsl, QueryResult,
};
use diesel_async::{AsyncConnection, RunQueryDsl};

use crate::{
    db_models::{message::Message, user_stat::UserStat},
    schema::{messages, user_stats},
    utils::database_utils::{get_config_table_chunk_size, ArcDbPool, MyDbConnection},
};

const POINT_PER_NEW_MESSAGE: i64 = 2;

async fn execute_create_message_events_sql(
    conn: &mut MyDbConnection,
    items_to_insert: Vec<Message>,
    user_new_message_counts: AHashMap<String, i64>,
) -> QueryResult<()> {
    conn.transaction(|conn| {
        Box::pin(async move {
            let create_message_query = diesel::insert_into(messages::table)
                .values(items_to_insert.clone())
                .on_conflict(messages::message_obj_addr)
                .do_nothing();
            create_message_query.execute(conn).await?;

            /*
            Do not try to backfill data (i.e. process same event twice), you would mess up the user stats.
            Instead, if you want to change the point calculation logic, you should delete all data and re-index from scratch.
            You can delete all data by revert all DB migrations, see README.md for more details.
             */

            let current_user_stats: AHashMap<String, UserStat> = user_stats::table
                .filter(user_stats::user_addr.eq_any(user_new_message_counts.keys()))
                .load::<UserStat>(conn)
                .await?
                .iter()
                .map(|user_point| (user_point.user_addr.clone(), user_point.clone()))
                .collect();
            let updated_user_stats = user_new_message_counts
                .iter()
                .map(
                    |(user_addr, new_message_count)| match current_user_stats.get(user_addr) {
                        Some(stat) => UserStat {
                            user_addr: user_addr.clone(),
                            creation_timestamp: stat.creation_timestamp,
                            last_update_timestamp: stat.last_update_timestamp,
                            user_point: stat.user_point + new_message_count * POINT_PER_NEW_MESSAGE,
                            created_messages: stat.created_messages + new_message_count,
                            updated_messages: stat.updated_messages,
                        },
                        None => UserStat {
                            user_addr: user_addr.clone(),
                            creation_timestamp: 0,
                            last_update_timestamp: 0,
                            user_point: new_message_count * POINT_PER_NEW_MESSAGE,
                            created_messages: *new_message_count,
                            updated_messages: 0,
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
