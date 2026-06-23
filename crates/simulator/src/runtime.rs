use config::ResolvedInstance;
use futures_util::{
    SinkExt,
    StreamExt,
};
use tokio_tungstenite::tungstenite::Message;

pub async fn run(
    config: ResolvedInstance,
) {
    let mut socket =
    match crate::cms::connect(&config).await {
        Ok(socket) => socket,
        Err(err) => {
            eprintln!(
                "{} connection failed: {}",
                config.instance.id,
                err,
            );

            return;
        }
    };

    let boot =
    crate::ocpp::boot_notification::build(
        &config
    );

    println!(
        "{} sending BootNotification",
        config.instance.id,
    );

    println!("{}", boot);

    if let Err(err) = socket
        .send(
            Message::Text(
                boot.to_string().into(),
            ),
        )
        .await
        {
            eprintln!(
                "{} failed to send BootNotification: {}",
                config.instance.id,
                err,
            );

            return;
        }

        loop {
            match socket.next().await {
                Some(Ok(Message::Ping(payload))) => {
                    println!(
                        "{} FRAME: Ping",
                        config.instance.id,
                    );

                    if let Err(err) = socket
                        .send(Message::Pong(payload))
                        .await
                        {
                            eprintln!(
                                "{} failed to send pong: {}",
                                config.instance.id,
                                err,
                            );

                            break;
                        }
                }

                Some(Ok(Message::Pong(_))) => {
                    println!(
                        "{} FRAME: Pong",
                        config.instance.id,
                    );
                }

                Some(Ok(Message::Text(text))) => {
                    let value: serde_json::Value =
                    match serde_json::from_str(&text) {
                        Ok(v) => v,
                        Err(err) => {
                            eprintln!(
                                "{} invalid JSON: {}",
                                config.instance.id,
                                err,
                            );

                            continue;
                        }
                    };

                    println!(
                        "{} OCPP: {:#}",
                        config.instance.id,
                        value,
                    );

                    let Some(array) =
                    value.as_array()
                    else {
                        continue;
                    };

                    if array.len() < 3 {
                        continue;
                    }

                    let message_type =
                    array[0].as_i64().unwrap_or(0);

                    println!(
                        "{} OCPP message type: {}",
                        config.instance.id,
                        message_type,
                    );

                    if message_type == 2 {
                        let message_id =
                        array[1]
                        .as_str()
                        .unwrap_or("");

                        let action =
                        array[2]
                        .as_str()
                        .unwrap_or("");

                        println!(
                            "{} OCPP action: {}",
                            config.instance.id,
                            action,
                        );

                        match action {
                            "TriggerMessage" => {
                                let connector_id =
                                array[3]
                                ["connectorId"]
                                .as_u64()
                                .unwrap_or(1)
                                as u8;

                                let requested =
                                array[3]
                                ["requestedMessage"]
                                .as_str()
                                .unwrap_or("");

                                println!(
                                    "{} TriggerMessage: {}",
                                    config.instance.id,
                                    requested,
                                );

                                let response =
                                serde_json::json!([
                                    3,
                                    message_id,
                                    {
                                        "status":
                                        "Accepted"
                                    }
                                ]);

                                println!(
                                    "{} sending TriggerMessage response",
                                    config.instance.id,
                                );

                                socket
                                .send(
                                    Message::Text(
                                        response
                                        .to_string()
                                        .into(),
                                    ),
                                )
                                .await
                                .unwrap();

                                if requested
                                    == "StatusNotification"
                                    {
                                        let status =
                                        crate::ocpp::status_notification::build(
                                            connector_id,
                                            crate::ocpp::types::ChargePointStatus::Available,
                                        );

                                        println!(
                                            "{} sending StatusNotification",
                                            config.instance.id,
                                        );

                                        println!(
                                            "{}",
                                            status,
                                        );

                                        socket
                                        .send(
                                            Message::Text(
                                                status
                                                .to_string()
                                                .into(),
                                            ),
                                        )
                                        .await
                                        .unwrap();
                                    }
                            }

                            _ => {
                                println!(
                                    "{} unhandled action: {}",
                                    config.instance.id,
                                    action,
                                );
                            }
                        }
                    }
                }

                Some(Ok(Message::Binary(data))) => {
                    println!(
                        "{} FRAME: Binary({} bytes)",
                             config.instance.id,
                             data.len(),
                    );
                }

                Some(Ok(Message::Close(frame))) => {
                    println!(
                        "{} FRAME: Close({:?})",
                             config.instance.id,
                             frame,
                    );

                    break;
                }

                Some(Ok(msg)) => {
                    println!(
                        "{} FRAME: {:?}",
                        config.instance.id,
                        msg,
                    );
                }

                Some(Err(err)) => {
                    eprintln!(
                        "{} socket error: {}",
                        config.instance.id,
                        err,
                    );

                    break;
                }

                None => {
                    println!(
                        "{} disconnected",
                        config.instance.id,
                    );

                    break;
                }
            }
        }
}
