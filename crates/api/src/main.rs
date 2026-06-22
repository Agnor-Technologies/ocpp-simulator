use rustls::crypto::ring;

use axum::{
    routing::get,
    Router,
};

async fn health() -> &'static str {
    "ok"
}

#[tokio::main]
async fn main() {
    ring::default_provider()
    .install_default()
    .expect("failed to install rustls provider");

    let configs = config::ConfigManager::load("configs")
        .expect("failed to load configs");

    println!("Loaded configs:");
    println!("{:#?}", configs);

    let mut manager =
        simulator::manager::SimulatorManager::new();

    for resolved in configs.resolve_all().unwrap() {
        manager.add_instance(resolved);
    }

    println!("Loaded simulator instances:");
    println!("{:#?}", manager);

    for instance in configs.instances.values() {
        manager
        .start_instance(&instance.id)
        .unwrap();
    }

    println!("Started instance:");


    let app = Router::new()
        .route("/health", get(health));

    let listener = tokio::net::TcpListener::bind(
        "127.0.0.1:3000",
    )
    .await
    .unwrap();

    println!("API listening on 3000");

    axum::serve(listener, app)
        .await
        .unwrap();
}
