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

    let boot_id =
    crate::ocpp::message_id::next();

    let boot =
    crate::ocpp::boot_notification::build(
        &config,
        &boot_id,
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

                    if let Some(array) = value.as_array() {
                        if array.len() >= 3 {
                            let message_type =
                            array[0].as_i64().unwrap_or(0);

                            if message_type == 3 {
                                let reply_id =
                                array[1].as_str().unwrap_or("");

                                if reply_id == boot_id {
                                    let status =
                                    array[2]["status"]
                                    .as_str()
                                    .unwrap_or("Unknown");

                                    println!(
                                        "{} BootNotification status: {}",
                                        config.instance.id,
                                        status,
                                    );

                                    if status != "Accepted" {
                                        eprintln!(
                                            "{} BootNotification rejected",
                                            config.instance.id,
                                        );

                                        break;
                                    }

                                    let interval =
                                    array[2]["interval"]
                                    .as_u64()
                                    .unwrap_or(60);

                                    println!(
                                        "{} heartbeat interval: {}s",
                                        config.instance.id,
                                        interval,
                                    );

                                    // Start heartbeat loop
                                    loop {
                                        tokio::time::sleep(
                                            std::time::Duration::from_secs(interval),
                                        )
                                        .await;

                                        let heartbeat =
                                        crate::ocpp::heartbeat::build();

                                        println!(
                                            "{} sending Heartbeat",
                                            config.instance.id,
                                        );

                                        if let Err(err) = socket
                                            .send(
                                                Message::Text(
                                                    heartbeat.to_string().into(),
                                                ),
                                            )
                                            .await
                                            {
                                                eprintln!(
                                                    "{} heartbeat failed: {}",
                                                    config.instance.id,
                                                    err,
                                                );

                                                break;
                                            }
                                    }
                                }
                            }
                        }
                    }

                    if let Err(err) =
                        crate::ocpp::dispatcher::handle(
                            &mut socket,
                            &value,
                        )
                        .await
                        {
                            eprintln!(
                                "{} dispatcher error: {}",
                                config.instance.id,
                                err,
                            );
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
