-- Your SQL goes here
CREATE TABLE
    user_stats (
        user_addr VARCHAR(300) NOT NULL UNIQUE PRIMARY KEY,
        creation_timestamp BIGINT NOT NULL,
        last_update_timestamp BIGINT NOT NULL,
        user_point BIGINT NOT NULL,
        created_messages BIGINT NOT NULL,
        updated_messages BIGINT NOT NULL
    );