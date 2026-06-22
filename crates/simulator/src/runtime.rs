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

    let boot = serde_json::json!([
        2,
        "boot-1",
        "BootNotification",
        {
            "chargePointVendor":
            config.profile.vendor,

            "chargePointModel":
            config.profile.model,

            "chargePointSerialNumber":
            config.instance.id,

            "chargeBoxSerialNumber":
            config.instance.id,

            "firmwareVersion":
            config.profile.firmware_version,

            "iccid":
            config.profile.iccid,

            "imsi":
            config.profile.imsi,

            "meterType":
            config.profile.meter_type,

            "meterSerialNumber":
            config.profile.meter_serial_number
        }
    ]);

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
                    println!(
                        "{} FRAME: Text({})",
                             config.instance.id,
                             text,
                    );
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
