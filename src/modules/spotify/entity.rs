use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthData {
    pub code: String,
    pub grant_type: String,
    pub redirect_uri: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AuthQuery {
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpotifyToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u32,
    pub refresh_token: Option<String>,
    pub scope: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenError {
    pub error: String,
    pub error_description: String,
}
