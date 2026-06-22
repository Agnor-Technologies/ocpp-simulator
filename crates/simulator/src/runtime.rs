use config::ResolvedInstance;
use crate::state::ChargerState;

pub async fn run(
    config: ResolvedInstance,
) {
    crate::cms::connect(&config)
    .await
    .unwrap();
    loop {
        println!(
            "{} state={:?}",
            config.instance.id,
            ChargerState::Available,
        );

        tokio::time::sleep(
            std::time::Duration::from_secs(10),
        )
        .await;
    }
}
