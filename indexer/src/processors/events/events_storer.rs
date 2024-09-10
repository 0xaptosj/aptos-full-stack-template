use crate::{
    db::common::models::events_models::{ContractEvent, Message},
    schema::{self},
    utils::database::{execute_in_chunks, get_config_table_chunk_size, ArcDbPool},
};
use ahash::AHashMap;
use anyhow::Result;
use aptos_indexer_processor_sdk::{
    traits::{async_step::AsyncRunType, AsyncStep, NamedStep, Processable},
    types::transaction_context::TransactionContext,
    utils::errors::ProcessorError,
};
use async_trait::async_trait;
use diesel::{
    pg::{upsert::excluded, Pg},
    query_builder::QueryFragment,
    query_dsl::methods::FilterDsl,
    BoolExpressionMethods, ExpressionMethods,
};
use tracing::error;

/// EventsStorer is a step that inserts events in the database.
pub struct EventsStorer
where
    Self: Sized + Send + 'static,
{
    conn_pool: ArcDbPool,
}

impl EventsStorer {
    pub fn new(conn_pool: ArcDbPool) -> Self {
        Self { conn_pool }
    }
}

fn create_message_events_sql(
    items_to_insert: Vec<Message>,
) -> (
    impl QueryFragment<Pg> + diesel::query_builder::QueryId + Send,
    Option<&'static str>,
) {
    use schema::messages::dsl::*;
    (
        diesel::insert_into(schema::messages::table)
            .values(items_to_insert)
            .on_conflict(message_obj_addr)
            .do_nothing(),
        None,
    )
}

fn update_message_events_sql(
    items_to_insert: Vec<Message>,
) -> (
    impl QueryFragment<Pg> + diesel::query_builder::QueryId + Send,
    Option<&'static str>,
) {
    use schema::messages::dsl::*;
    (
        diesel::insert_into(schema::messages::table)
            .values(items_to_insert)
            .on_conflict(message_obj_addr)
            .do_update()
            .set((
                message_obj_addr.eq(excluded(message_obj_addr)),
                creator_addr.eq(excluded(creator_addr)),
                creation_timestamp.eq(excluded(creation_timestamp)),
                last_update_timestamp.eq(excluded(last_update_timestamp)),
                last_update_event_idx.eq(excluded(last_update_event_idx)),
                content.eq(excluded(content)),
            ))
            .filter(
                // Update only if the last update timestamp is greater than the existing one
                // or if the last update timestamp is the same but the event index is greater
                last_update_timestamp
                    .lt(excluded(last_update_timestamp))
                    .or(last_update_timestamp
                        .eq(excluded(last_update_timestamp))
                        .and(last_update_event_idx.lt(excluded(last_update_event_idx)))),
            ),
        None,
    )
}

#[async_trait]
impl Processable for EventsStorer {
    type Input = ContractEvent;
    type Output = ContractEvent;
    type RunType = AsyncRunType;

    async fn process(
        &mut self,
        events: TransactionContext<ContractEvent>,
    ) -> Result<Option<TransactionContext<ContractEvent>>, ProcessorError> {
        let per_table_chunk_sizes: AHashMap<String, usize> = AHashMap::new();
        let (create_events, update_events) = events.clone().data.into_iter().fold(
            (vec![], vec![]),
            |(mut create_events, mut update_events), event| {
                match event {
                    ContractEvent::CreateMessageEvent(message) => {
                        create_events.push(message);
                    }
                    ContractEvent::UpdateMessageEvent(message) => {
                        update_events.push(message);
                    }
                }
                (create_events, update_events)
            },
        );

        let create_result = execute_in_chunks(
            self.conn_pool.clone(),
            create_message_events_sql,
            &create_events,
            get_config_table_chunk_size::<Message>("messages", &per_table_chunk_sizes),
        )
        .await;

        match create_result {
            Ok(_) => {}
            Err(e) => {
                error!("Failed to store create message events: {:?}", e);
                return Err(ProcessorError::ProcessError {
                    message: e.to_string(),
                });
            }
        }

        let update_result = execute_in_chunks(
            self.conn_pool.clone(),
            update_message_events_sql,
            &update_events,
            // run update sequentially because we cannot update one record multiple times in a single DB transaction
            1,
        )
        .await;

        match update_result {
            Ok(_) => {}
            Err(e) => {
                error!("Failed to store update message events: {:?}", e);
                return Err(ProcessorError::ProcessError {
                    message: e.to_string(),
                });
            }
        }

        Ok(Some(events))
    }
}

impl AsyncStep for EventsStorer {}

impl NamedStep for EventsStorer {
    fn name(&self) -> String {
        "EventsStorer".to_string()
    }
}
