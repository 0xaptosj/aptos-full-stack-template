-- Your SQL goes here
CREATE TABLE messages (
    id BIGSERIAL,
    message_obj_addr VARCHAR(300) NOT NULL,
    creator_addr VARCHAR(300) NOT NULL,
    creation_timestamp BIGINT NOT NULL,
    creation_tx_version BIGINT NOT NULL,
    update_timestamp BIGINT,
    update_tx_version BIGINT,
    content TEXT NOT NULL,
    PRIMARY KEY (id)
);