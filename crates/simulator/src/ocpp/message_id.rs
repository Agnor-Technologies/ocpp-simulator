use std::time::{
    SystemTime,
    UNIX_EPOCH,
};

pub fn next() -> String {
    SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_millis()
    .to_string()
}
