use std::error::Error;

use actix_web::{get, web, HttpResponse};

use crate::modules::duolingo::User;

#[get("/duo/stats/{name}")]
async fn get_duo_user(path: web::Path<String>) -> Result<HttpResponse, Box<dyn Error>> {
    let name = path.into_inner();
    let res: String = reqwest::Client::new()
        .get(format!("https://www.duolingo.com/users/{}", name))
        .header("Authorization", "")
        .send()
        .await?
        .text()
        .await?;

    let user: User = serde_json::from_str(&res).unwrap();
    Ok(HttpResponse::Ok().json(user))
}
