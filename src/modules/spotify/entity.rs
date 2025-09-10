use serde::{Deserialize, Serialize};
use serde_json::Value;
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotifyResponse {
    pub status: String,
    pub data: Root,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub timestamp: i64,
    pub context: Value,
    #[serde(rename = "progress_ms")]
    pub progress_ms: i64,
    pub item: Item,
    #[serde(rename = "currently_playing_type")]
    pub currently_playing_type: String,
    #[serde(rename = "is_playing")]
    pub is_playing: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub album: Album,
    pub artists: Vec<Artist>,
    #[serde(rename = "available_markets")]
    pub available_markets: Vec<String>,
    #[serde(rename = "disc_number")]
    pub disc_number: i64,
    #[serde(rename = "duration_ms")]
    pub duration_ms: i64,
    pub explicit: bool,
    #[serde(rename = "external_ids")]
    pub external_ids: ExternalIds,
    #[serde(rename = "external_urls")]
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    #[serde(rename = "is_local")]
    pub is_local: bool,
    pub name: String,
    pub popularity: i64,
    #[serde(rename = "preview_url")]
    pub preview_url: String,
    #[serde(rename = "track_number")]
    pub track_number: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub uri: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Album {
    #[serde(rename = "album_type")]
    pub album_type: String,
    pub artists: Vec<Artist>,
    #[serde(rename = "available_markets")]
    pub available_markets: Vec<String>,
    #[serde(rename = "external_urls")]
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    #[serde(rename = "release_date")]
    pub release_date: String,
    #[serde(rename = "release_date_precision")]
    pub release_date_precision: String,
    #[serde(rename = "total_tracks")]
    pub total_tracks: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub uri: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artist {
    #[serde(rename = "external_urls")]
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub uri: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalUrls {
    pub spotify: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub height: i64,
    pub url: String,
    pub width: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalIds {
    pub isrc: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub scope: String,
}

// Enhanced real-time song information structures
#[derive(Debug, Serialize, Deserialize)]
pub struct RealTimeSongInfo {
    pub track: TrackInfo,
    pub playback: PlaybackInfo,
    pub device: Option<DeviceInfo>,
    pub context: Option<ContextInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackInfo {
    pub id: String,
    pub name: String,
    pub artists: Vec<Artist>,
    pub album: Album,
    pub duration_ms: i64,
    pub explicit: bool,
    pub popularity: i64,
    pub preview_url: Option<String>,
    pub external_urls: ExternalUrls,
    pub images: Vec<Image>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaybackInfo {
    pub is_playing: bool,
    pub progress_ms: i64,
    pub timestamp: i64,
    pub currently_playing_type: String,
    pub repeat_state: String,
    pub shuffle_state: bool,
    pub volume_percent: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub id: String,
    pub is_active: bool,
    pub is_private_session: bool,
    pub is_restricted: bool,
    pub name: String,
    pub device_type: String,
    pub volume_percent: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContextInfo {
    pub uri: String,
    pub href: String,
    pub external_urls: ExternalUrls,
    pub context_type: String,
}

// Player state structure (read-only for dashboard)
#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerState {
    pub device: Option<DeviceInfo>,
    pub repeat_state: String,
    pub shuffle_state: bool,
    pub context: Option<ContextInfo>,
    pub timestamp: i64,
    pub progress_ms: i64,
    pub is_playing: bool,
    pub item: Option<Item>,
    pub currently_playing_type: String,
}

// Queue management structures
#[derive(Debug, Serialize, Deserialize)]
pub struct QueueInfo {
    pub currently_playing: Option<Item>,
    pub queue: Vec<Item>,
}

// WebSocket real-time update structure
#[derive(Debug, Serialize, Deserialize)]
pub struct RealTimeUpdate {
    pub event_type: String,
    pub timestamp: i64,
    pub data: serde_json::Value,
}
