-- Your SQL goes here
CREATE TABLE messages (
    id BIGSERIAL,
    message_obj_addr VARCHAR(300) NOT NULL UNIQUE,
    creator_addr VARCHAR(300) NOT NULL,
    creation_timestamp BIGINT NOT NULL,
    last_update_timestamp BIGINT NOT NULL,
    -- we store the UpdateMessageEvent index so when we update in batch we can make sure we don't overwrite newer data
    last_update_event_idx BIGINT NOT NULL,
    content TEXT NOT NULL,
    PRIMARY KEY (id)
);

CREATE INDEX idx_message_obj_addr ON messages (message_obj_addr);