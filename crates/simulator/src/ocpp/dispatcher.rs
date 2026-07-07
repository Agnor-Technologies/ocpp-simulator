use anyhow::Result;
use serde_json::Value;
use tokio_tungstenite::tungstenite::Message;
use futures_util::SinkExt;

use crate::ocpp::{
    status_notification,
    trigger_message,
    types::ChargePointStatus,
};

pub async fn handle<S>(
    socket: &mut S,
    value: &Value,
) -> Result<()>
where
    S: SinkExt<Message> + Unpin,
    S::Error: std::error::Error + Send + Sync + 'static,
{
    let Some(array) = value.as_array() else {
        return Ok(());
    };

    if array.len() < 3 {
        return Ok(());
    }

    let message_type =
        array[0].as_i64().unwrap_or(0);

    if message_type != 2 {
        return Ok(());
    }

    let message_id =
        array[1].as_str().unwrap_or("");

    let action =
        array[2].as_str().unwrap_or("");

    match action {
        "TriggerMessage" => {
            let connector_id =
                array[3]["connectorId"]
                    .as_u64()
                    .unwrap_or(1) as u8;

            let requested =
                trigger_message::parse(value);

            let response = serde_json::json!([
                3,
                message_id,
                {
                    "status":"Accepted"
                }
            ]);

            socket
                .send(Message::Text(
                    response.to_string().into(),
                ))
                .await?;

            match requested {
                trigger_message::TriggerMessageType::StatusNotification => {
                    let status =
                        status_notification::build(
                            connector_id,
                            ChargePointStatus::Available,
                        );

                    socket
                        .send(Message::Text(
                            status.to_string().into(),
                        ))
                        .await?;
                }

                _ => {}
            }
        }

        _ => {}
    }

    Ok(())
}
