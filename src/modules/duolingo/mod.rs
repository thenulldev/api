use redis::{aio::ConnectionManager, AsyncCommands};

use crate::db::redis::RedisManager;

use self::entity::User;

pub mod entity;
pub mod handler;

#[derive(Clone)]
pub struct DuoManager;
impl DuoManager {
    pub async fn check_duo_stats(redis: &mut RedisManager, name: &str) -> bool {
        // let data = &state.lock().await;
        // let redis = data.redis.clone();
        // let mut redis = Self::get_redis(cm).await;
        let res: bool = redis
            .cm
            .exists(format!("duo:stats:{}", name))
            .await
            .unwrap();
        return res;
    }

    pub async fn get_duo_stats(redis: &mut RedisManager, name: &str) -> User {
        let current = redis::cmd("GET")
            .arg(format!("duo:stats:{}", name))
            .query_async::<ConnectionManager, String>(&mut redis.cm)
            .await
            .unwrap();

        let json: User = serde_json::from_str(&current).unwrap();

        return json;
    }

    pub async fn store_duo_stats(redis: &mut RedisManager, name: &String, data: &User) {
        let json = serde_json::to_string(data).unwrap();

        redis::cmd("SET")
            .arg(format!("duo:stats:{}", name))
            .arg(json)
            .arg("EX")
            // TODO Figure out better TTL
            .arg(3000)
            .query_async::<ConnectionManager, String>(&mut redis.cm)
            .await
            .unwrap();
    }
}
