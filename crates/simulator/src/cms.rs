use anyhow::Result;
use config::ResolvedInstance;
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async,
    tungstenite::client::IntoClientRequest,
    MaybeTlsStream,
    WebSocketStream,
};

pub async fn connect(
    config: &ResolvedInstance,
) -> Result<
WebSocketStream<
MaybeTlsStream<TcpStream>
>
> {
    let url = format!(
        "{}/{}?token={}",
        config.cms.base_url.trim_end_matches('/'),
        config.instance.id,
        config.instance.token,
    );

    println!(
        "Connecting {} to {}",
        config.instance.id,
        url,
    );

    let mut request =
    url.into_client_request()?;

    request.headers_mut().insert(
        "Sec-WebSocket-Protocol",
        "ocpp1.6".parse()?,
    );

    let (socket, response) =
    connect_async(request).await?;

    println!(
        "{} connected ({})",
             config.instance.id,
             response.status(),
    );

    Ok(socket)
}
