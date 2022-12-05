use envconfig::Envconfig;
use reqwest::header::{ACCEPT, AUTHORIZATION, USER_AGENT};

use crate::{config::Config, db::redis::RedisManager};

use self::entity::{RepoResponse, RunnerResponse};

pub mod entity;
pub mod handler;

#[derive(Clone)]
pub struct GithubManager;
impl GithubManager {
    pub async fn get_runners(redis: &mut RedisManager) -> Result<RunnerResponse, reqwest::Error> {
        let config = Config::init_from_env().unwrap();

        let res = reqwest::Client::new()
            .get("https://api.github.com/orgs/thenulldev/actions/runners")
            .header(AUTHORIZATION, format!("Bearer {}", &config.github_secret))
            .header(ACCEPT, "application/vnd.github+json")
            .header(USER_AGENT, "NullDev-API")
            .send()
            .await
            .unwrap();
        let body = &res.text().await.unwrap();
        // println!("{:?}", &body);
        let body: RunnerResponse = serde_json::from_str(&body).unwrap();
        Ok(body)
    }

    pub async fn get_repos(redis: &mut RedisManager) -> Result<RepoResponse, reqwest::Error> {
        let config = Config::init_from_env().unwrap();

        let res = reqwest::Client::new()
            .get("https://api.github.com/orgs/thenulldev/repos")
            .header(AUTHORIZATION, format!("Bearer {}", &config.github_secret))
            .header(ACCEPT, "application/vnd.github+json")
            .header(USER_AGENT, "NullDev-API")
            .send()
            .await
            .unwrap();
        let body = &res.text().await.unwrap();
        // println!("{:?}", &body);
        let body: RepoResponse = serde_json::from_str(&body).unwrap();
        Ok(body)
    }
}
