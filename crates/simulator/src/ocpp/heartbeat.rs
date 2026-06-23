use serde_json::json;
use serde_json::Value;

pub fn build() -> Value {
    json!([
        2,
        crate::ocpp::message_id::next(),
          "Heartbeat",
          {}
    ])
}
