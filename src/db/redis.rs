use envconfig::Envconfig;
use log::info;
use redis::{aio::ConnectionManager, Client};

use crate::{config::Config, error::AppError};

#[derive(Clone)]
pub struct RedisManager {
    pub connection: ConnectionManager,
}

impl RedisManager {
    pub async fn new() -> Result<Self, AppError> {
        let config = Config::init_from_env()?;
        let client = Client::open(config.redis)?;
        let connection = ConnectionManager::new(client).await?;
        info!("Connected to Redis");
        Ok(Self { connection })
    }
}
