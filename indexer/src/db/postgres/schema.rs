// @generated automatically by Diesel CLI.

diesel::table! {
    events (transaction_version, event_index) {
        sequence_number -> Int8,
        creation_number -> Int8,
        #[max_length = 66]
        account_address -> Varchar,
        transaction_version -> Int8,
        transaction_block_height -> Int8,
        #[sql_name = "type"]
        type_ -> Text,
        data -> Jsonb,
        inserted_at -> Timestamp,
        event_index -> Int8,
        #[max_length = 300]
        indexed_type -> Varchar,
    }
}

diesel::table! {
    ledger_infos (chain_id) {
        chain_id -> Int8,
    }
}

diesel::table! {
    messages (id) {
        id -> Int8,
        #[max_length = 300]
        message_obj_addr -> Varchar,
        #[max_length = 300]
        creator_addr -> Varchar,
        creation_tx_version -> Int8,
        last_update_tx_version -> Nullable<Int8>,
        content -> Text,
    }
}

diesel::table! {
    processor_status (processor) {
        #[max_length = 50]
        processor -> Varchar,
        last_success_version -> Int8,
        last_updated -> Timestamp,
        last_transaction_timestamp -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    events,
    ledger_infos,
    messages,
    processor_status,
);
