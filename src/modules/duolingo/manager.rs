use redis::{aio::ConnectionManager, AsyncCommands};

use crate::{db::redis::RedisManager, error::AppError};

use super::entity::User;

#[derive(Clone)]
pub struct DuoManager;

impl DuoManager {
    pub async fn check_duo_stats(redis: &mut RedisManager, name: &str) -> Result<bool, AppError> {
        let exists: bool = redis
            .connection
            .exists(format!("duo:stats:{}", name))
            .await?;
        Ok(exists)
    }

    pub async fn get_duo_stats(redis: &mut RedisManager, name: &str) -> Result<User, AppError> {
        let data: String = redis::cmd("GET")
            .arg(format!("duo:stats:{}", name))
            .query_async::<ConnectionManager, String>(&mut redis.connection)
            .await?;

        let user: User = serde_json::from_str(&data)?;
        Ok(user)
    }

    pub async fn store_duo_stats(
        redis: &mut RedisManager,
        name: &str,
        data: &User,
    ) -> Result<(), AppError> {
        let json = serde_json::to_string(data)?;

        redis::cmd("SET")
            .arg(format!("duo:stats:{}", name))
            .arg(json)
            .arg("EX")
            .arg(3000) // 50 minutes TTL
            .query_async::<ConnectionManager, String>(&mut redis.connection)
            .await?;

        Ok(())
    }
}
