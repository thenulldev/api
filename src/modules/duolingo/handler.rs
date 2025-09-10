use actix_web::{get, web, HttpResponse};
use envconfig::Envconfig;
use log::info;
use tokio::sync::Mutex;

use crate::{
    client::NullClient,
    config::Config,
    error::AppError,
    modules::duolingo::{entity::User, DuoManager},
};

#[get("/v1/duo/stats/{name}")]
async fn get_duo_user(
    path: web::Path<String>,
    state: web::Data<Mutex<NullClient>>,
) -> Result<HttpResponse, AppError> {
    let data = state.lock().await;
    let mut redis = data.redis.clone();
    let config = Config::init_from_env()?;
    let name = path.into_inner();
    
    drop(data); // Release the lock early
    
    if DuoManager::check_duo_stats(&mut redis, &name).await? {
        info!("Retrieved cached stats for user: {}", &name);
        let user = DuoManager::get_duo_stats(&mut redis, &name).await?;
        Ok(HttpResponse::Ok().json(user))
    } else {
        info!("Fetching fresh stats for user: {}", &name);
        let response = reqwest::Client::new()
            .get(format!("https://www.duolingo.com/users/{}", name))
            .header("Authorization", config.duo_api)
            .send()
            .await?
            .text()
            .await?;

        let user: User = serde_json::from_str(&response)?;
        
        // Store stats in cache
        DuoManager::store_duo_stats(&mut redis, &name, &user).await?;
        Ok(HttpResponse::Ok().json(user))
    }
}
