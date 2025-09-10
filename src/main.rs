use client::NullClient;
use dotenvy::dotenv;
use log::error;
// Import Modules
pub mod client;
pub mod config;
pub mod db;
pub mod error;
pub mod modules;

// Main Application Loop
#[rustfmt::skip]
#[tokio::main]
async fn main() {
    dotenv().ok();
    // Init Logger
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    // Start Client
    if let Err(e) = NullClient::start().await {
        error!("Failed to start server: {}", e);
        std::process::exit(1);
    }
}
