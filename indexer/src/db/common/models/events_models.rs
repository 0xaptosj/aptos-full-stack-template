#![allow(clippy::extra_unused_lifetimes)]

use crate::schema::messages;
use aptos_indexer_processor_sdk::{
    aptos_protos::transaction::v1::Event as EventPB, utils::convert::standardize_address,
};
use diesel::{AsChangeset, Insertable};
use field_count::FieldCount;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MoveObj {
    pub inner: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
/// On-chain representation of a message
pub struct MessageOnChain {
    pub creator: String,
    pub content: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
/// On-chain representation of a message creation event
pub struct CreateMessageEventOnChain {
    pub message_obj: MoveObj,
    pub message: MessageOnChain,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
/// On-chain representation of a message update event
pub struct UpdateMessageEventOnChain {
    pub message_obj: MoveObj,
    pub message: MessageOnChain,
}

#[derive(Clone, Debug, Deserialize, FieldCount, Insertable, Serialize)]
#[diesel(table_name = messages)]
pub struct CreateMessageEvent {
    pub message_obj_addr: String,
    pub creator_addr: String,
    pub creation_tx_version: i64,
    pub content: String,
}

#[derive(AsChangeset, Clone, Debug, Deserialize, FieldCount, Insertable, Serialize)]
#[diesel(table_name = messages)]
pub struct UpdateMessageEvent {
    pub message_obj_addr: String,
    pub last_update_tx_version: i64,
    pub content: String,
}

#[derive(Clone)]
pub enum ContractEvent {
    CreateMessageEvent(CreateMessageEvent),
    UpdateMessageEvent(UpdateMessageEvent),
}

impl ContractEvent {
    pub fn from_event(
        contract_address: &str,
        event: &EventPB,
        transaction_version: i64,
    ) -> Option<Self> {
        let t: &str = event.type_str.as_ref();
        let should_include = t.starts_with(contract_address);

        if should_include {
            if t.starts_with(
                format!("{}::message_board::CreateMessageEvent", contract_address).as_str(),
            ) {
                println!("CreateMessageEvent {}", event.data.as_str());
                let create_message_event_on_chain: CreateMessageEventOnChain =
                    serde_json::from_str(event.data.as_str()).expect(
                        format!(
                            "Failed to parse CreateMessageEvent, {}",
                            event.data.as_str()
                        )
                        .as_str(),
                    );
                let create_message_event = CreateMessageEvent {
                    message_obj_addr: standardize_address(
                        create_message_event_on_chain.message_obj.inner.as_str(),
                    ),
                    creator_addr: standardize_address(
                        create_message_event_on_chain.message.creator.as_str(),
                    ),
                    creation_tx_version: transaction_version,
                    content: create_message_event_on_chain.message.content,
                };
                Some(ContractEvent::CreateMessageEvent(create_message_event))
            } else if t.starts_with(
                format!("{}::message_board::UpdateMessageEvent", contract_address).as_str(),
            ) {
                println!("UpdateMessageEvent {}", event.data.as_str());
                let update_message_event_on_chain: UpdateMessageEventOnChain =
                    serde_json::from_str(event.data.as_str()).expect(
                        format!(
                            "Failed to parse UpdateMessageEvent, {}",
                            event.data.as_str()
                        )
                        .as_str(),
                    );
                let update_message_event = UpdateMessageEvent {
                    message_obj_addr: standardize_address(
                        update_message_event_on_chain.message_obj.inner.as_str(),
                    ),
                    last_update_tx_version: transaction_version,
                    content: update_message_event_on_chain.message.content,
                };
                Some(ContractEvent::UpdateMessageEvent(update_message_event))
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn from_events(
        contract_address: &str,
        events: &[EventPB],
        transaction_version: i64,
    ) -> Vec<Self> {
        events
            .iter()
            .enumerate()
            .map(|(_, event)| Self::from_event(contract_address, event, transaction_version))
            .filter_map(|event| event)
            .collect()
    }
}
