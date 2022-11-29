use std::error::Error;

use actix_web::{get, web, HttpResponse};
use envconfig::Envconfig;
use log::info;
use tokio::sync::Mutex;

use crate::{
    client::NullClient,
    config::Config,
    modules::duolingo::{entity::User, DuoManager},
};

#[get("/v1/duo/stats/{name}")]
async fn get_duo_user(
    path: web::Path<String>,
    state: web::Data<Mutex<NullClient>>,
) -> Result<HttpResponse, Box<dyn Error>> {
    let data = &state.lock().await;
    let mut redis = data.redis.clone();
    let config = Config::init_from_env().unwrap();
    let name = path.into_inner();
    // DuoManager::check_duo_stats(data, &name);
    if DuoManager::check_duo_stats(&mut redis, &name).await {
        info!("Got cached stats for user {}", &name);
        let user = DuoManager::get_duo_stats(&mut redis, &name).await;
        Ok(HttpResponse::Ok().json(user))
    } else {
        info!("Fetched stats for user {}", &name);
        let res: String = reqwest::Client::new()
            .get(format!("https://www.duolingo.com/users/{}", name))
            .header("Authorization", config.duo_api)
            .send()
            .await?
            .text()
            .await?;

        let user: User = serde_json::from_str(&res).unwrap();
        // Store stats in cache
        DuoManager::store_duo_stats(&mut redis, &name, &user).await;
        Ok(HttpResponse::Ok().json(user))
    }
}
