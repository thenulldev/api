use envconfig::Envconfig;
use log::info;
use redis::{aio::ConnectionManager, AsyncCommands};
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};

use crate::{config::Config, db::redis::RedisManager, error::AppError};

use super::entity::{
    DeviceInfo, PlayerState, QueueInfo, RealTimeSongInfo, SpotifyToken, TokenResponse,
};

#[derive(Clone)]
pub struct SpotifyManager;

impl SpotifyManager {
    pub async fn store_spotify_creds(redis: &mut RedisManager, data: &SpotifyToken) -> Result<(), AppError> {
        redis::cmd("SET")
            .arg("spotify:access_token")
            .arg(&data.access_token)
            .arg("EX")
            .arg(data.expires_in)
            .query_async::<ConnectionManager, String>(&mut redis.connection)
            .await?;

        if let Some(refresh_token) = &data.refresh_token {
            redis::cmd("SET")
                .arg("spotify:refresh_token")
                .arg(refresh_token)
                .query_async::<ConnectionManager, String>(&mut redis.connection)
                .await?;
        }

        Ok(())
    }

    pub async fn renew_spotify_access(redis: &mut RedisManager) -> Result<bool, AppError> {
        let exists: bool = redis.connection.exists("spotify:refresh_token").await?;
        Ok(exists)
    }

    pub async fn check_spotify_auth(redis: &mut RedisManager) -> Result<bool, AppError> {
        let exists: bool = redis.connection.exists("spotify:refresh_token").await?;
        Ok(exists)
    }

    pub async fn check_spotify_access(redis: &mut RedisManager) -> Result<bool, AppError> {
        let exists: bool = redis.connection.exists("spotify:access_token").await?;
        Ok(exists)
    }

    pub async fn get_spotify_current(redis: &mut RedisManager) -> Result<(), AppError> {
        if Self::check_spotify_access(redis).await? {
            let token: String = redis.connection.get("spotify:access_token").await?;

            let response = reqwest::Client::new()
                .get("https://api.spotify.com/v1/me/player/currently-playing")
                .header(AUTHORIZATION, format!("Bearer {}", token))
                .header(CONTENT_TYPE, "application/json")
                .header(ACCEPT, "application/json")
                .send()
                .await?;
                
            let status = response.status().as_u16();
            let content_length = response.content_length().unwrap_or(0);

            match status {
                200 => {
                    info!("Currently playing");
                    if content_length > 0 {
                        let text = response.text().await?;
                        redis::cmd("SET")
                            .arg("spotify:now_playing")
                            .arg(text)
                            .arg("EX")
                            .arg(300) // 5 minutes TTL
                            .query_async::<ConnectionManager, String>(&mut redis.connection)
                            .await?;
                    }
                }
                204 => {
                    info!("Nothing currently playing");
                }
                401 => {
                    info!("Access token expired, refreshing");
                    let token_response = Self::refresh_access_token(redis).await?;

                    redis::cmd("SET")
                        .arg("spotify:access_token")
                        .arg(token_response.access_token)
                        .arg("EX")
                        .arg(token_response.expires_in)
                        .query_async::<ConnectionManager, String>(&mut redis.connection)
                        .await?;
                }
                429 => {
                    info!("Rate limit exceeded");
                }
                _ => {
                    info!("Unexpected status code: {}", status);
                }
            }
        }
        Ok(())
    }

    async fn refresh_access_token(
        redis: &mut RedisManager,
    ) -> Result<TokenResponse, AppError> {
        let config = Config::init_from_env()?;
        let token: String = redis.connection.get("spotify:refresh_token").await?;
        
        let params = [
            ("grant_type", "refresh_token"),
            ("refresh_token", token.as_str()),
            ("client_id", config.spotify_client_id.as_str()),
            ("client_secret", config.spotify_client_secret.as_str()),
        ];

        let response = reqwest::Client::new()
            .post("https://accounts.spotify.com/api/token")
            .form(&params)
            .send()
            .await?;
            
        let response_text = response.text().await?;
        let token_response: TokenResponse = serde_json::from_str(&response_text)?;
        Ok(token_response)
    }

    // Enhanced real-time song information methods
    pub async fn get_real_time_song_info(redis: &mut RedisManager) -> Result<RealTimeSongInfo, AppError> {
        if !Self::check_spotify_access(redis).await? {
            return Err(AppError::SpotifyError("Not authenticated".to_string()));
        }

        let token: String = redis.connection.get("spotify:access_token").await?;
        
        // Get player state
        let player_response = reqwest::Client::new()
            .get("https://api.spotify.com/v1/me/player")
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .header(ACCEPT, "application/json")
            .send()
            .await?;

        if player_response.status() == 204 {
            return Err(AppError::SpotifyError("No active device".to_string()));
        }

        let player_state: PlayerState = player_response.json().await?;
        
        if let Some(item) = player_state.item {
            let album_images = item.album.images.clone();
            let track_info = RealTimeSongInfo {
                track: super::entity::TrackInfo {
                    id: item.id,
                    name: item.name,
                    artists: item.artists,
                    album: item.album,
                    duration_ms: item.duration_ms,
                    explicit: item.explicit,
                    popularity: item.popularity,
                    preview_url: Some(item.preview_url),
                    external_urls: item.external_urls,
                    images: album_images,
                },
                playback: super::entity::PlaybackInfo {
                    is_playing: player_state.is_playing,
                    progress_ms: player_state.progress_ms,
                    timestamp: player_state.timestamp,
                    currently_playing_type: player_state.currently_playing_type,
                    repeat_state: player_state.repeat_state,
                    shuffle_state: player_state.shuffle_state,
                    volume_percent: player_state.device.as_ref().map(|d| d.volume_percent),
                },
                device: player_state.device,
                context: player_state.context,
            };

            // Cache the real-time info
            let cache_data = serde_json::to_string(&track_info)?;
            redis::cmd("SET")
                .arg("spotify:realtime_info")
                .arg(cache_data)
                .arg("EX")
                .arg(30) // 30 seconds TTL for real-time data
                .query_async::<ConnectionManager, String>(&mut redis.connection)
                .await?;

            Ok(track_info)
        } else {
            Err(AppError::SpotifyError("No track playing".to_string()))
        }
    }

    // Get available devices
    pub async fn get_devices(redis: &mut RedisManager) -> Result<Vec<DeviceInfo>, AppError> {
        if !Self::check_spotify_access(redis).await? {
            return Err(AppError::SpotifyError("Not authenticated".to_string()));
        }

        let token: String = redis.connection.get("spotify:access_token").await?;
        
        let response = reqwest::Client::new()
            .get("https://api.spotify.com/v1/me/player/devices")
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .header(ACCEPT, "application/json")
            .send()
            .await?;

        let devices_response: serde_json::Value = response.json().await?;
        let devices: Vec<DeviceInfo> = serde_json::from_value(devices_response["devices"].clone())?;
        
        Ok(devices)
    }

    // Get current queue
    pub async fn get_queue(redis: &mut RedisManager) -> Result<QueueInfo, AppError> {
        if !Self::check_spotify_access(redis).await? {
            return Err(AppError::SpotifyError("Not authenticated".to_string()));
        }

        let token: String = redis.connection.get("spotify:access_token").await?;
        
        let response = reqwest::Client::new()
            .get("https://api.spotify.com/v1/me/player/queue")
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .header(ACCEPT, "application/json")
            .send()
            .await?;

        let queue_info: QueueInfo = response.json().await?;
        Ok(queue_info)
    }

    // Get user's top tracks (for dashboard analytics)
    pub async fn get_top_tracks(redis: &mut RedisManager, time_range: &str, limit: i32) -> Result<serde_json::Value, AppError> {
        if !Self::check_spotify_access(redis).await? {
            return Err(AppError::SpotifyError("Not authenticated".to_string()));
        }

        let token: String = redis.connection.get("spotify:access_token").await?;
        
        let url = format!(
            "https://api.spotify.com/v1/me/top/tracks?time_range={}&limit={}&offset=0",
            time_range, limit
        );

        let response = reqwest::Client::new()
            .get(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .header(ACCEPT, "application/json")
            .send()
            .await?;

        let tracks_data: serde_json::Value = response.json().await?;
        Ok(tracks_data)
    }

    // Get user's top artists (for dashboard analytics)
    pub async fn get_top_artists(redis: &mut RedisManager, time_range: &str, limit: i32) -> Result<serde_json::Value, AppError> {
        if !Self::check_spotify_access(redis).await? {
            return Err(AppError::SpotifyError("Not authenticated".to_string()));
        }

        let token: String = redis.connection.get("spotify:access_token").await?;
        
        let url = format!(
            "https://api.spotify.com/v1/me/top/artists?time_range={}&limit={}&offset=0",
            time_range, limit
        );

        let response = reqwest::Client::new()
            .get(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .header(ACCEPT, "application/json")
            .send()
            .await?;

        let artists_data: serde_json::Value = response.json().await?;
        Ok(artists_data)
    }

    // Get user's recently played tracks
    pub async fn get_recently_played(redis: &mut RedisManager, limit: i32) -> Result<serde_json::Value, AppError> {
        if !Self::check_spotify_access(redis).await? {
            return Err(AppError::SpotifyError("Not authenticated".to_string()));
        }

        let token: String = redis.connection.get("spotify:access_token").await?;
        
        let url = format!(
            "https://api.spotify.com/v1/me/player/recently-played?limit={}",
            limit
        );

        let response = reqwest::Client::new()
            .get(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .header(ACCEPT, "application/json")
            .send()
            .await?;

        let recent_data: serde_json::Value = response.json().await?;
        Ok(recent_data)
    }

    // Get user's playlists
    pub async fn get_user_playlists(redis: &mut RedisManager, limit: i32) -> Result<serde_json::Value, AppError> {
        if !Self::check_spotify_access(redis).await? {
            return Err(AppError::SpotifyError("Not authenticated".to_string()));
        }

        let token: String = redis.connection.get("spotify:access_token").await?;
        
        let url = format!(
            "https://api.spotify.com/v1/me/playlists?limit={}&offset=0",
            limit
        );

        let response = reqwest::Client::new()
            .get(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .header(ACCEPT, "application/json")
            .send()
            .await?;

        let playlists_data: serde_json::Value = response.json().await?;
        Ok(playlists_data)
    }
}
