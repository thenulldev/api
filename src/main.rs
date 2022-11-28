use actix_web::{middleware::Logger, web, App, HttpServer};
use config::Config;
use db::{postgres::PostgresManager, redis::RedisManager};
use envconfig::Envconfig;
use helpers::duolingo::get_duo_user;
use tokio::sync::Mutex;
pub mod config;
pub mod db;
pub mod helpers;
pub mod modules;
pub struct State {
    pub redis: RedisManager,
    pub postgres: PostgresManager,
}

#[rustfmt::skip]
#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Init Logger
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    // Init Redis
    let redis = RedisManager::new().await;
    // Init Postgres
    let postgres = PostgresManager::new().await;
    // Init Config
    let config = Config::init_from_env().unwrap();

    // Store data in State
    let data = web::Data::new(Mutex::new(State {
        postgres,
        redis,
    }));

    // Start HTTP Server
    HttpServer::new(move || {
        let logger = Logger::default();
        App::new().wrap(logger).service(get_duo_user).app_data(web::Data::clone(&data))
    })
    .bind(((config.listen_host).to_owned(), config.listen_port))?
    .run()
    .await
}
