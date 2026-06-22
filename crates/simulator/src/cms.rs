use anyhow::Result;
use config::ResolvedInstance;
use tokio_tungstenite::connect_async;

pub async fn connect(
    config: &ResolvedInstance,
) -> Result<()> {
    let url = format!(
        "{}/?token={}",
        config.cms.base_url,
        config.instance.token
    );

    let (_socket, _) =
    connect_async(url).await?;

    println!(
        "{} connected",
        config.instance.id
    );

    Ok(())
}
