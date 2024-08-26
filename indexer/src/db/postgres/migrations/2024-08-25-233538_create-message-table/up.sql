-- Your SQL goes here
CREATE TABLE messages (
    id BIGSERIAL,
    message_obj_addr VARCHAR(300) NOT NULL,
    creator_addr VARCHAR(300) NOT NULL,
    creation_timestamp BIGINT NOT NULL,
    content TEXT NOT NULL,
    PRIMARY KEY (id)
);