use std::time::{SystemTime, UNIX_EPOCH};

pub fn current_unix_timestamp_in_seconds() -> i64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_secs() as i64
}
