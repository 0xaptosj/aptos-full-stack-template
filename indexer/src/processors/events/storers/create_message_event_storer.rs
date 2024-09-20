use ahash::AHashMap;
use anyhow::Result;
use aptos_indexer_processor_sdk::utils::errors::ProcessorError;
use diesel::{pg::Pg, query_builder::QueryFragment, ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;
use tracing::error;

use crate::{
    db_models::events_models::{Message, UserPoint},
    schema::{messages, user_points},
    utils::{
        database_execution::execute_in_chunks,
        database_utils::{get_config_table_chunk_size, ArcDbPool, MyDbConnection},
    },
};

async fn create_message_events_sql(
    conn: &mut MyDbConnection,
    items_to_insert: Vec<Message>,
) -> Vec<impl QueryFragment<Pg> + diesel::query_builder::QueryId + Send> {
    let mut queries = vec![];
    let create_message_query = diesel::insert_into(messages::table)
        .values(items_to_insert.clone())
        .on_conflict(messages::message_obj_addr)
        .do_nothing();
    queries.push(create_message_query);

    let users = items_to_insert
        .iter()
        .map(|message| message.creator_addr.clone())
        .collect::<Vec<String>>();

    let current_user_points = user_points::table
        .filter(user_points::user_addr.eq_any(users))
        .load::<UserPoint>(conn)
        .await;

    // for message in items_to_insert {
    // let update_user_point_query = user_points::table
    //     .filter(user_points::user_addr.eq(&message.creator_addr))
    //     .first::<UserPoint>(conn)
    //     .await
    //     .optional();
    // let update_user_point_query = match update_user_point_query {
    //     Ok(Some(current_point)) => {
    //         let new_point = current_point.points + 1;
    //         diesel::update(user_points::table.filter(user_points::user_addr.eq(&message.creator_addr)))
    //             .set(user_points::points.eq(new_point))
    //     }
    //     Ok(None) => {
    //         diesel::insert_into(user_points::table)
    //             .values(UserPoint {
    //                 user_addr: message.creator_addr,
    //                 points: 1,
    //                 creation_timestamp: message.creation_timestamp,
    //                 last_update_timestamp: message.creation_timestamp,
    //                 last_update_event_idx: message.last_update_event_idx,
    //             })
    //             .on_conflict(user_points::user_addr)
    //             .do_nothing()
    //     }
    //     Err(e) => {
    //         error!("Failed to get user point: {:?}", e);
    //         return Err(ProcessorError::ProcessError {
    //             message: e.to_string(),
    //         });
    //     }
    // };
    // queries.push(update_user_point_query);
    // }

    // let current_point = user_points::table
    //     .filter(user_points::user_addr.eq(processor_name))
    //     .first::<Self>(conn)
    //     .await
    //     .optional();
    // let update_user_point_query =

    queries
}

pub async fn process_create_message_events(
    pool: ArcDbPool,
    per_table_chunk_sizes: AHashMap<String, usize>,
    create_events: Vec<Message>,
) -> Result<(), ProcessorError> {
    let create_result = execute_in_chunks(
        pool.clone(),
        create_message_events_sql,
        &create_events,
        get_config_table_chunk_size::<Message>("messages", &per_table_chunk_sizes),
    )
    .await;

    match create_result {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("Failed to store create message events: {:?}", e);
            Err(ProcessorError::ProcessError {
                message: e.to_string(),
            })
        }
    }
}
