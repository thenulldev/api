use actix_web::{get, http::Error, web, HttpResponse};
use envconfig::Envconfig;
use serde_json::json;
use tokio::sync::Mutex;

use crate::{
    client::NullClient,
    config::Config,
    modules::spotify::{
        entity::{AuthData, AuthQuery, SpotifyToken, TokenError},
        SpotifyManager,
    },
};

#[get("/v1/spotify")]
async fn current(data: web::Data<Mutex<NullClient>>) -> Result<HttpResponse, Error> {
    let data = data.lock().await;
    let _redis = &mut data.redis.clone();

    // let current = SpotifyManager::get_spotify_current().await;

    Ok(HttpResponse::Ok()
        .insert_header(("Content-Type", "application/json"))
        .body(json!({"success": true, "data": "TEST"}).to_string()))
}

#[get("/v1/spotify/auth")]
async fn authorize(data: web::Data<Mutex<NullClient>>) -> Result<HttpResponse, Error> {
    let data = data.lock().await;
    let _redis = &mut data.redis.clone();
    //TODO Check if user has already been authorized
    let config = Config::init_from_env().unwrap();

    let scope = "user-read-playback-state+user-read-currently-playing";
    let redirect_uri = "http://127.0.0.1:8080/v1/spotify/callback";
    let url = format!("https://accounts.spotify.com/authorize?client_id={}&response_type=code&scope={}&redirect_uri={}", config.spotify_client_id, scope, redirect_uri);
    let json = json!({ "url": url });
    Ok(HttpResponse::Ok()
        .append_header(("Content-type", "application/json"))
        .json(json))
}

#[get("/v1/spotify/callback")]
async fn callback(
    data: web::Data<Mutex<NullClient>>,
    info: web::Query<AuthQuery>,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let data = data.lock().await;
    let mut redis = &mut data.redis.clone();
    let config = Config::init_from_env().unwrap();

    let code = &info.code;
    let redirect_uri = "http://127.0.0.1:8080/v1/spotify/callback";
    let data = AuthData {
        code: code.into(),
        grant_type: "authorization_code".into(),
        redirect_uri: redirect_uri.into(),
    };

    let data =
        serde_urlencoded::to_string(&data).expect("error serializing data for spotify token");

    let res = reqwest::Client::new()
        .post(format!("https://accounts.spotify.com/api/token?{}", data))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Content-Length", "0")
        .basic_auth(config.spotify_client_id, Some(config.spotify_client_secret))
        .send()
        .await?
        .text()
        .await?;

    if !res.is_empty() {
        let body: SpotifyToken = serde_json::from_str(&res).unwrap();
        SpotifyManager::store_spotify_creds(&mut redis, &body).await;
        Ok(HttpResponse::NoContent().finish())
    } else {
        let body: TokenError = serde_json::from_str(&res).unwrap();
        Ok(HttpResponse::InternalServerError().json(body))
    }
}
