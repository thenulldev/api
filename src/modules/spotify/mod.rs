use envconfig::Envconfig;
use log::info;
use redis::{aio::ConnectionManager, AsyncCommands};
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};

use crate::{config::Config, db::redis::RedisManager};

use self::entity::{SpotifyToken, TokenResponse};

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

    pub async fn renew_spotify_access(redis: &mut RedisManager) -> String {
        let res: String = redis.cm.exists("spotify:refresh_token").await.unwrap();
        return res;
    }

    pub async fn check_spotify_auth(redis: &mut RedisManager) -> bool {
        let res: bool = redis.cm.exists("spotify:refresh_token").await.unwrap();
        return res;
    }

    pub async fn check_spotify_access(redis: &mut RedisManager) -> bool {
        let access_token: bool = redis.cm.exists("spotify:access_token").await.unwrap();
        return access_token;
    }

    pub async fn get_spotify_current(redis: &mut RedisManager) {
        if Self::check_spotify_access(redis).await {
            let token: String = redis.cm.get("spotify:access_token").await.unwrap();

            let res = reqwest::Client::new()
                .get("https://api.spotify.com/v1/me/player/currently-playing")
                .header(AUTHORIZATION, format!("Bearer {}", token))
                .header(CONTENT_TYPE, "application/json")
                .header(ACCEPT, "application/json")
                .send()
                .await
                .unwrap();
            let status = &res.status().as_u16();
            let length = &res.content_length().unwrap();
            let empty: u64 = 0;

            match status {
                200 => {
                    info!("Playing");
                    if length > &empty {
                        redis::cmd("SET")
                            .arg("spotify:now_playing")
                            .arg(&res.text().await.unwrap())
                            .arg("EX")
                            // TODO Figure out better TTL
                            .arg(300)
                            .query_async::<ConnectionManager, String>(&mut redis.cm)
                            .await
                            .unwrap();
                    }
                }
                204 => {
                    info!("NOTHING PLAYING");
                    // println!("{:?}", content);
                }
                401 => {
                    info!("AUTH TOKEN NOT VALID");
                    // TODO Renew token
                    let token = Self::refresh_access_token(redis).await.unwrap();

                    redis::cmd("SET")
                        .arg("spotify:access_token")
                        .arg(token.access_token)
                        .arg("EX")
                        .arg(token.expires_in)
                        .query_async::<ConnectionManager, String>(&mut redis.cm)
                        .await
                        .unwrap();
                }
                429 => {
                    info!("RATE LIMIT EXCEEDED");
                }
                _ => {
                    println!("Other status")
                }
            }
        }
    }

    async fn refresh_access_token(
        redis: &mut RedisManager,
    ) -> Result<TokenResponse, reqwest::Error> {
        let config = Config::init_from_env().unwrap();
        let token: String = redis.cm.get("spotify:refresh_token").await.unwrap();
        let params = [
            ("grant_type", "refresh_token"),
            ("refresh_token", token.as_str()),
            ("client_id", config.spotify_client_id.as_str()),
            ("client_secret", config.spotify_client_secret.as_str()),
        ];

        let resp = reqwest::Client::new()
            .post("https://accounts.spotify.com/api/token")
            .form(&params)
            .send()
            .await?;
        let body: TokenResponse = serde_json::from_str(&resp.text().await.unwrap()).unwrap();
        Ok(body)
    }
}
