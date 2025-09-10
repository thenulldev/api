use actix_web::{get, web, HttpResponse};
use envconfig::Envconfig;
use redis::aio::ConnectionManager;
use serde_json::json;
use tokio::sync::Mutex;

use crate::{
    client::NullClient,
    config::Config,
    error::AppError,
    modules::spotify::{
        entity::{AuthData, AuthQuery, Root, SpotifyToken, TokenError},
        SpotifyManager,
    },
};

#[get("/v1/spotify")]
async fn current(data: web::Data<Mutex<NullClient>>) -> Result<HttpResponse, AppError> {
    let data = data.lock().await;
    let mut redis = data.redis.clone();
    drop(data); // Release the lock early
    
    SpotifyManager::get_spotify_current(&mut redis).await?;
    let current: String = redis::cmd("GET")
        .arg("spotify:now_playing")
        .query_async::<ConnectionManager, String>(&mut redis.connection)
        .await?;
    let playing: Root = serde_json::from_str(&current)?;
    Ok(HttpResponse::Ok()
        .insert_header(("Content-Type", "application/json"))
        .body(json!({"success": true, "data": playing}).to_string()))
}

#[get("/v1/spotify/auth")]
async fn authorize(data: web::Data<Mutex<NullClient>>) -> Result<HttpResponse, AppError> {
    let data = data.lock().await;
    let mut redis = data.redis.clone();
    drop(data); // Release the lock early

    if !SpotifyManager::check_spotify_access(&mut redis).await? {
        let config = Config::init_from_env()?;

        let scope = "user-read-playback-state+user-read-currently-playing";
        let redirect_uri = config.spotify_redirect_uri;
        let url = format!("https://accounts.spotify.com/authorize?client_id={}&response_type=code&scope={}&redirect_uri={}", config.spotify_client_id, scope, redirect_uri);
        let json = json!({ "info": "Click the URL to authorize the app", "url": url });
        Ok(HttpResponse::Ok()
            .append_header(("Content-type", "application/json"))
            .json(json))
    } else {
        let json = json!({ "error": "Application already authorized!" });
        Ok(HttpResponse::Ok()
            .append_header(("Content-type", "application/json"))
            .json(json))
    }
}

#[get("/v1/spotify/callback")]
async fn callback(
    data: web::Data<Mutex<NullClient>>,
    info: web::Query<AuthQuery>,
) -> Result<HttpResponse, AppError> {
    let data = data.lock().await;
    let mut redis = data.redis.clone();
    drop(data); // Release the lock early
    
    let config = Config::init_from_env()?;

    let code = &info.code;
    let redirect_uri = config.spotify_redirect_uri;
    let auth_data = AuthData {
        code: code.into(),
        grant_type: "authorization_code".into(),
        redirect_uri: redirect_uri.into(),
    };

    let form_data = serde_urlencoded::to_string(&auth_data)?;

    let response = reqwest::Client::new()
        .post(format!("https://accounts.spotify.com/api/token?{}", form_data))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Content-Length", "0")
        .basic_auth(config.spotify_client_id, Some(config.spotify_client_secret))
        .send()
        .await?
        .text()
        .await?;

    if !response.is_empty() {
        let token: SpotifyToken = serde_json::from_str(&response)?;
        SpotifyManager::store_spotify_creds(&mut redis, &token).await?;
        Ok(HttpResponse::NoContent().finish())
    } else {
        let error: TokenError = serde_json::from_str(&response)?;
        Ok(HttpResponse::InternalServerError().json(error))
    }
}

// Enhanced real-time song information endpoints
#[get("/v1/spotify/realtime")]
async fn realtime_info(data: web::Data<Mutex<NullClient>>) -> Result<HttpResponse, AppError> {
    let data = data.lock().await;
    let mut redis = data.redis.clone();
    drop(data);

    let song_info = SpotifyManager::get_real_time_song_info(&mut redis).await?;
    Ok(HttpResponse::Ok()
        .insert_header(("Content-Type", "application/json"))
        .json(json!({
            "success": true,
            "data": song_info,
            "timestamp": chrono::Utc::now().timestamp()
        })))
}

#[get("/v1/spotify/devices")]
async fn devices(data: web::Data<Mutex<NullClient>>) -> Result<HttpResponse, AppError> {
    let data = data.lock().await;
    let mut redis = data.redis.clone();
    drop(data);

    let devices = SpotifyManager::get_devices(&mut redis).await?;
    Ok(HttpResponse::Ok()
        .insert_header(("Content-Type", "application/json"))
        .json(json!({
            "success": true,
            "data": devices,
            "count": devices.len()
        })))
}

#[get("/v1/spotify/queue")]
async fn queue(data: web::Data<Mutex<NullClient>>) -> Result<HttpResponse, AppError> {
    let data = data.lock().await;
    let mut redis = data.redis.clone();
    drop(data);

    let queue_info = SpotifyManager::get_queue(&mut redis).await?;
    Ok(HttpResponse::Ok()
        .insert_header(("Content-Type", "application/json"))
        .json(json!({
            "success": true,
            "data": queue_info
        })))
}

// Dashboard analytics endpoints
#[get("/v1/spotify/top/tracks")]
async fn top_tracks(
    data: web::Data<Mutex<NullClient>>,
    query: web::Query<TopTracksQuery>
) -> Result<HttpResponse, AppError> {
    let data = data.lock().await;
    let mut redis = data.redis.clone();
    drop(data);

    let tracks_data = SpotifyManager::get_top_tracks(&mut redis, &query.time_range, query.limit).await?;
    Ok(HttpResponse::Ok()
        .insert_header(("Content-Type", "application/json"))
        .json(json!({
            "success": true,
            "data": tracks_data,
            "time_range": query.time_range,
            "limit": query.limit
        })))
}

#[get("/v1/spotify/top/artists")]
async fn top_artists(
    data: web::Data<Mutex<NullClient>>,
    query: web::Query<TopArtistsQuery>
) -> Result<HttpResponse, AppError> {
    let data = data.lock().await;
    let mut redis = data.redis.clone();
    drop(data);

    let artists_data = SpotifyManager::get_top_artists(&mut redis, &query.time_range, query.limit).await?;
    Ok(HttpResponse::Ok()
        .insert_header(("Content-Type", "application/json"))
        .json(json!({
            "success": true,
            "data": artists_data,
            "time_range": query.time_range,
            "limit": query.limit
        })))
}

#[get("/v1/spotify/recently-played")]
async fn recently_played(
    data: web::Data<Mutex<NullClient>>,
    query: web::Query<RecentlyPlayedQuery>
) -> Result<HttpResponse, AppError> {
    let data = data.lock().await;
    let mut redis = data.redis.clone();
    drop(data);

    let recent_data = SpotifyManager::get_recently_played(&mut redis, query.limit).await?;
    Ok(HttpResponse::Ok()
        .insert_header(("Content-Type", "application/json"))
        .json(json!({
            "success": true,
            "data": recent_data,
            "limit": query.limit
        })))
}

#[get("/v1/spotify/playlists")]
async fn playlists(
    data: web::Data<Mutex<NullClient>>,
    query: web::Query<PlaylistsQuery>
) -> Result<HttpResponse, AppError> {
    let data = data.lock().await;
    let mut redis = data.redis.clone();
    drop(data);

    let playlists_data = SpotifyManager::get_user_playlists(&mut redis, query.limit).await?;
    Ok(HttpResponse::Ok()
        .insert_header(("Content-Type", "application/json"))
        .json(json!({
            "success": true,
            "data": playlists_data,
            "limit": query.limit
        })))
}

// Query parameter structures for dashboard endpoints
#[derive(serde::Deserialize)]
pub struct TopTracksQuery {
    #[serde(default = "default_time_range")]
    pub time_range: String,
    #[serde(default = "default_limit")]
    pub limit: i32,
}

#[derive(serde::Deserialize)]
pub struct TopArtistsQuery {
    #[serde(default = "default_time_range")]
    pub time_range: String,
    #[serde(default = "default_limit")]
    pub limit: i32,
}

#[derive(serde::Deserialize)]
pub struct RecentlyPlayedQuery {
    #[serde(default = "default_limit")]
    pub limit: i32,
}

#[derive(serde::Deserialize)]
pub struct PlaylistsQuery {
    #[serde(default = "default_limit")]
    pub limit: i32,
}

// Default values for query parameters
fn default_time_range() -> String {
    "medium_term".to_string()
}

fn default_limit() -> i32 {
    20
}

