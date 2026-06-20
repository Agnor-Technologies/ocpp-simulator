use axum::{
    routing::get,
    Router,
};

async fn health() -> &'static str {
    "ok"
}

#[tokio::main]
async fn main() {
    let cfg = config::load("configs/example.toml")
        .expect("failed to load config");

    println!("Loaded config:");
    println!("{:#?}", cfg);

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
