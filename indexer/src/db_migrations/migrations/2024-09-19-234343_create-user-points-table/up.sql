-- Your SQL goes here
CREATE TABLE user_points (
    user_addr VARCHAR(300) NOT NULL UNIQUE PRIMARY KEY,
    creation_timestamp BIGINT NOT NULL,
    last_update_timestamp BIGINT NOT NULL,
    points BIGINT NOT NULL,
    -- we store the event index so when we update in batch,
    -- we ignore when the event index is less than the last update event index
    last_update_event_idx BIGINT NOT NULL
);