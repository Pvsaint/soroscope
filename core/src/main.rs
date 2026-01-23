use config::{Config, ConfigError};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct AppConfig {
    server_port: u16,
    rust_log: String,
    soroban_rpc_url: String,
}

fn load_config() -> Result<AppConfig, ConfigError> {
    // Load .env file if present
    dotenvy::dotenv().ok();

    let settings = Config::builder()
        .add_source(config::Environment::default())
        .set_default("server_port", 3000)?
        .set_default("rust_log", "info")?
        .set_default("soroban_rpc_url", "https://soroban-testnet.stellar.org")?
        .build()?;

    settings.try_deserialize()
}

fn main() {
    let config = load_config().expect("Failed to load configuration");
    println!("SoroScope CLI Initialized with config: {:?}", config);
mod errors;

use crate::errors::AppError;
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route(
            "/error",
            get(|| async { Err::<&str, AppError>(AppError::BadRequest("Test error".to_string())) }),
        );

    // run it with hyper on localhost:3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
