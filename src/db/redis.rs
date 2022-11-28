use envconfig::Envconfig;
use log::info;
use redis::{aio::ConnectionManager, AsyncCommands, Client};
extern crate serde_json;
use crate::{config::Config, modules::duolingo::User};

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

    pub async fn check_duo_stats(&mut self, name: &str) -> bool {
        let res: bool = self.cm.exists(format!("duo:stats:{}", name)).await.unwrap();
        return res;
    }

    pub async fn get_duo_stats(&mut self, name: &str) -> User {
        let current = redis::cmd("GET")
            .arg(format!("duo:stats:{}", name))
            .query_async::<ConnectionManager, String>(&mut self.cm)
            .await
            .unwrap();

        let json: User = serde_json::from_str(&current).unwrap();

        return json;
    }

    pub async fn store_duo_stats(&mut self, name: &String, data: &User) {
        let json = serde_json::to_string(data).unwrap();

        redis::cmd("SET")
            .arg(format!("duo:stats:{}", name))
            .arg(json)
            .arg("EX")
            // TODO Figure out better TTL
            .arg(3000)
            .query_async::<ConnectionManager, String>(&mut self.cm)
            .await
            .unwrap();
    }
}
