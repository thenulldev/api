// let conn = Database::connect(&db_url).await.unwrap();

use std::time::Duration;

use envconfig::Envconfig;
use log::info;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::config::Config;

pub struct PostgresManager {
    pub pm: DatabaseConnection,
}

impl PostgresManager {
    pub async fn new() -> Self {
        let config = Config::init_from_env().unwrap();
        let mut opt = ConnectOptions::new(config.db_url.to_owned());
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(false)
            .sqlx_logging_level(log::LevelFilter::Info)
            .set_schema_search_path("test".into()); // Setting default PostgreSQL schema

        let postgres = Database::connect(opt).await.unwrap();
        info!("Connected to Database");
        Self { pm: postgres }
    }
}
