use envconfig::Envconfig;
use log::info;
use redis::{aio::ConnectionManager, Client};

use crate::config::Config;

#[derive(Clone)]
pub struct RedisManager {
    pub cm: ConnectionManager,
}

impl RedisManager {
    pub async fn new() -> Self {
        let config = Config::init_from_env().unwrap();
        let client = Client::open(config.redis).unwrap();
        let cm = ConnectionManager::new(client).await.unwrap();
        info!("Connected to Redis");
        Self { cm }
    }
}
