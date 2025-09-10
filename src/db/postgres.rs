use envconfig::Envconfig;
use log::info;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::{config::Config, error::AppError};

pub struct PostgresManager {
    pub pool: Pool<Postgres>,
}

impl PostgresManager {
    pub async fn new() -> Result<Self, AppError> {
        let config = Config::init_from_env()?;

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&config.db_url)
            .await?;

        info!("Connected to PostgreSQL database");
        Ok(Self { pool })
    }
}
