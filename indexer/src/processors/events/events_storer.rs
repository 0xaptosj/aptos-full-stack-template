use crate::{
    db::common::models::events_models::{
        ContractEvent, CreateMessageEvent, Message, UpdateMessageEvent,
    },
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
    ExpressionMethods,
};
use tracing::{error, info};

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

fn create_message_events_query(
    items_to_insert: Vec<CreateMessageEvent>,
) -> (
    impl QueryFragment<Pg> + diesel::query_builder::QueryId + Send,
    Option<&'static str>,
) {
    use schema::messages::dsl::*;
    (
        diesel::insert_into(schema::messages::table)
            .values(items_to_insert.into_iter().map(|event| {
                (
                    message_obj_addr.eq(event.message_obj_addr),
                    creator_addr.eq(event.creator_addr),
                    creation_timestamp.eq(event.creation_timestamp),
                    creation_tx_version.eq(event.creation_tx_version),
                    content.eq(event.content),
                    update_timestamp.eq(None::<i64>),
                    update_tx_version.eq(None::<i64>),
                )
            }))
            .on_conflict(message_obj_addr)
            .do_nothing(),
        None,
    )
}

fn update_message_events_query(
    items_to_insert: Vec<UpdateMessageEvent>,
) -> (
    impl QueryFragment<Pg> + diesel::query_builder::QueryId + Send,
    Option<&'static str>,
) {
    use schema::messages::dsl::*;
    (
        diesel::update(schema::messages::table)
            .filter(
                message_obj_addr
                    .eq_any(items_to_insert.iter().map(|event| &event.message_obj_addr)),
            )
            .set((
                content.eq(diesel::dsl::any(
                    items_to_insert.iter().map(|event| &event.content),
                )),
                update_timestamp.eq(diesel::dsl::any(
                    items_to_insert
                        .iter()
                        .map(|event| event.update_timestamp as i64),
                )),
                update_tx_version.eq(diesel::dsl::any(
                    items_to_insert
                        .iter()
                        .map(|event| event.update_tx_version as i64),
                )),
            )),
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
        let (create_events, update_events) = events.data.into_iter().fold(
            (vec![], vec![]),
            |(mut create_events, mut update_events), event| {
                match event {
                    ContractEvent::CreateMessageEvent(create_message_event) => {
                        create_events.push(create_message_event);
                    }
                    ContractEvent::UpdateMessageEvent(update_message_event) => {
                        update_events.push(update_message_event);
                    }
                }
                (create_events, update_events)
            },
        );

        let create_result = execute_in_chunks(
            self.conn_pool.clone(),
            create_message_events_query,
            &create_events,
            get_config_table_chunk_size::<CreateMessageEvent>(
                "create_message_events",
                &per_table_chunk_sizes,
            ),
        )
        .await;

        let update_result = execute_in_chunks(
            self.conn_pool.clone(),
            update_message_events_query,
            &update_events,
            get_config_table_chunk_size::<UpdateMessageEvent>(
                "update_message_events",
                &per_table_chunk_sizes,
            ),
        )
        .await;

        match (create_result, update_result) {
            (Ok(_), Ok(_)) => {
                info!(
                    "Events version [{}, {}] stored successfully",
                    events.start_version, events.end_version
                );
            }
            (Err(e), _) | (_, Err(e)) => {
                error!("Failed to store events: {:?}", e);
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
