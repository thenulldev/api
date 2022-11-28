use std::error::Error;

use actix_web::{get, web, HttpResponse};
use envconfig::Envconfig;
use tokio::sync::Mutex;

use crate::{config::Config, modules::duolingo::User, State};

#[get("/duo/stats/{name}")]
async fn get_duo_user(
    path: web::Path<String>,
    data: web::Data<Mutex<State>>,
) -> Result<HttpResponse, Box<dyn Error>> {
    let data = data.lock().await;
    let redis = &mut data.redis.clone();
    let config = Config::init_from_env().unwrap();
    let name = path.into_inner();

    if redis.check_duo_stats(&name).await {
        let user = redis.get_duo_stats(&name).await;
        Ok(HttpResponse::Ok().json(user))
    } else {
        // TODO Make call cached
        let res: String = reqwest::Client::new()
            .get(format!("https://www.duolingo.com/users/{}", name))
            .header("Authorization", config.duo_api)
            .send()
            .await?
            .text()
            .await?;

        let user: User = serde_json::from_str(&res).unwrap();

        // Store stats in cache
        redis.store_duo_stats(&name, &user).await;
        Ok(HttpResponse::Ok().json(user))
    }
    // println!("{}", redis.check_duo_stats(&name).await);
}
