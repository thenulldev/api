use actix_web::{middleware::Logger, web, App, HttpServer};
use envconfig::Envconfig;
use log::info;
use tokio::sync::Mutex;

use crate::{
    config::Config,
    db::{postgres::PostgresManager, redis::RedisManager},
    error::AppError,
    modules::{
        default,
        duolingo::handler::get_duo_user,
        github::handler::{repos, runners},
        health, index,
        spotify::handler::{
            authorize, callback, current, devices, playlists, queue, 
            realtime_info, recently_played, top_artists, top_tracks
        },
    },
};

pub struct NullClient {
    pub redis: RedisManager,
    pub postgres: PostgresManager,
}

impl NullClient {
    // Start the Client
    pub async fn start() -> Result<(), AppError> {
        // Init Redis
        let redis = RedisManager::new().await?;
        // Init Postgres
        let postgres = PostgresManager::new().await?;

        // Store data in State
        let data = web::Data::new(Mutex::new(Self { postgres, redis }));
        // Load Client Config
        let config = Config::init_from_env()?;
        
        // Start HTTP Server
        let server = HttpServer::new(move || {
            let logger = Logger::default();
            App::new()
                .wrap(logger)
                .default_service(web::route().to(default))
                .configure(Self::init)
                .app_data(web::Data::clone(&data))
        })
        .bind((config.listen_host.clone(), config.listen_port))?;
        
        info!(
            "Server started and listening on http://{}:{}",
            config.listen_host, config.listen_port
        );
        
        server.run().await.map_err(AppError::from)
    }

    // Initialize Services
    pub fn init(cfg: &mut web::ServiceConfig) {
        // General
        cfg.service(index);
        cfg.service(health);
        //Spotify
        cfg.service(current);
        cfg.service(authorize);
        cfg.service(callback);
        cfg.service(realtime_info);
        cfg.service(devices);
        cfg.service(queue);
        cfg.service(top_tracks);
        cfg.service(top_artists);
        cfg.service(recently_played);
        cfg.service(playlists);
        // Github
        cfg.service(runners);
        cfg.service(repos);
        //Duolingo
        cfg.service(get_duo_user);
    }
}
