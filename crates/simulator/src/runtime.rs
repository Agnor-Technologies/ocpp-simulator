use config::ResolvedInstance;
use crate::state::ChargerState;

pub async fn run(
    config: ResolvedInstance,
) {
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
