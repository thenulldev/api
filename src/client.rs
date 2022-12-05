use actix_web::{middleware::Logger, web, App, HttpServer};
use envconfig::Envconfig;
use log::info;
use tokio::sync::Mutex;

use crate::{
    config::Config,
    db::{postgres::PostgresManager, redis::RedisManager},
    modules::{
        default,
        duolingo::handler::get_duo_user,
        github::handler::{repos, runners},
        health, index,
        spotify::handler::{authorize, callback, current},
    },
};

pub struct NullClient {
    pub redis: RedisManager,
    pub postgres: PostgresManager,
}

impl NullClient {
    // Start the Client
    pub async fn start() -> std::io::Result<()> {
        // Init Redis
        let redis = RedisManager::new().await;
        // Init Postgres
        let postgres = PostgresManager::new().await;

        // Store data in State
        let data = web::Data::new(Mutex::new(Self { postgres, redis }));
        // Load Client Config
        let config = Config::init_from_env().unwrap();
        // Start HTTP Server
        let server = HttpServer::new(move || {
            let logger = Logger::default();
            App::new()
                .wrap(logger)
                .default_service(web::route().to(default))
                .configure(Self::init)
                .app_data(web::Data::clone(&data))
        })
        .bind(((config.listen_host).to_owned(), config.listen_port))?;
        info!(
            "Connected and listening on http://{}:{}",
            (config.listen_host).to_owned(),
            config.listen_port
        );
        server.run().await
    }

    // Initialize Services
    pub fn init(cfg: &mut web::ServiceConfig) {
        cfg.service(get_duo_user);
        cfg.service(authorize);
        cfg.service(callback);
        cfg.service(index);
        cfg.service(health);
        cfg.service(current);
        cfg.service(runners);
        cfg.service(repos);
    }
}
