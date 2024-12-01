use std::time::{SystemTime, UNIX_EPOCH};

pub fn current_timestamp() -> u128 {
    let now = SystemTime::now();
    now.duration_since(UNIX_EPOCH).unwrap().as_millis()
}