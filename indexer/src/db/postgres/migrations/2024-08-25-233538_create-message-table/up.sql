-- Your SQL goes here
CREATE TABLE messages (
    id BIGSERIAL,
    message_obj_addr VARCHAR(300) NOT NULL UNIQUE,
    creator_addr VARCHAR(300) NOT NULL,
    creation_tx_version BIGINT NOT NULL,
    last_update_tx_version BIGINT,
    content TEXT NOT NULL,
    PRIMARY KEY (id)
);

CREATE INDEX idx_message_obj_addr ON messages (message_obj_addr);