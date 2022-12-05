use client::NullClient;
use dotenvy::dotenv;
// Import Modules
pub mod client;
pub mod config;
pub mod db;
pub mod modules;

// Main Application Loop
#[rustfmt::skip]
#[tokio::main]
async fn main(){
    dotenv().ok();
    // Init Logger
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    // Start Client
    NullClient::start().await;
}
