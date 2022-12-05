use actix_web::{get, http::Error, web, HttpResponse};
use serde_json::json;
use tokio::sync::Mutex;

use crate::{client::NullClient, modules::github::GithubManager};

#[get("/v1/github/runners")]
async fn runners(data: web::Data<Mutex<NullClient>>) -> Result<HttpResponse, Error> {
    let data = data.lock().await;
    let redis = &mut data.redis.clone();
    let runners = GithubManager::get_runners(redis).await.unwrap();
    // TODO Add Caching
    Ok(HttpResponse::Ok()
        .insert_header(("Content-Type", "application/json"))
        .body(json!({"success": true, "data": runners}).to_string()))
}

#[get("/v1/github/repos")]
async fn repos(data: web::Data<Mutex<NullClient>>) -> Result<HttpResponse, Error> {
    let data = data.lock().await;
    let redis = &mut data.redis.clone();
    let repos = GithubManager::get_repos(redis).await.unwrap();
    // TODO Add Caching
    Ok(HttpResponse::Ok()
        .insert_header(("Content-Type", "application/json"))
        .body(json!({"success": true, "data": repos}).to_string()))
}
