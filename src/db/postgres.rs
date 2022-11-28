// let conn = Database::connect(&db_url).await.unwrap();

use envconfig::Envconfig;
use log::info;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::config::Config;

pub struct PostgresManager {
    pub pm: Pool<Postgres>,
}

impl PostgresManager {
    pub async fn new() -> Self {
        let config = Config::init_from_env().unwrap();

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&config.db_url)
            .await
            .unwrap();

        info!("Connected to Database");
        Self { pm: pool }
    }
}
