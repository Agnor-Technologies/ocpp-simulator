use serde_json::Value;

pub enum TriggerMessageType {
    StatusNotification,
    Heartbeat,
    BootNotification,
    Unknown,
}

pub fn parse(
    value: &Value,
) -> TriggerMessageType {
    let requested =
    value[3]["requestedMessage"]
    .as_str()
    .unwrap_or("");

    match requested {
        "StatusNotification" =>
        TriggerMessageType::StatusNotification,

        "Heartbeat" =>
        TriggerMessageType::Heartbeat,

        "BootNotification" =>
        TriggerMessageType::BootNotification,

        _ =>
        TriggerMessageType::Unknown,
    }
}
