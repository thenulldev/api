use redis::aio::ConnectionManager;

use crate::db::redis::RedisManager;

use self::entity::SpotifyToken;

pub mod entity;
pub mod handler;

#[derive(Clone)]
pub struct SpotifyManager;
impl SpotifyManager {
    pub async fn store_spotify_creds(redis: &mut RedisManager, data: &SpotifyToken) {
        redis::cmd("SET")
            .arg("spotify:access_token")
            .arg(&data.access_token)
            .arg("EX")
            .arg(data.expires_in)
            .query_async::<ConnectionManager, String>(&mut redis.cm)
            .await
            .unwrap();

        match &data.refresh_token {
            Some(refresh_token) => {
                redis::cmd("SET")
                    .arg("spotify:refresh_token")
                    .arg(refresh_token)
                    .query_async::<ConnectionManager, String>(&mut redis.cm)
                    .await
                    .unwrap();
            }
            None => (),
        }
    }
}
