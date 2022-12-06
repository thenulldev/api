use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "LISTEN_HOST", default = "0.0.0.0")]
    pub listen_host: String,

    #[envconfig(from = "LISTEN_PORT", default = "8080")]
    pub listen_port: u16,

    #[envconfig(
        from = "DB_URL",
        default = "postgresql://postgres:db@localhost:5432/api"
    )]
    pub db_url: String,

    #[envconfig(from = "REDIS_URL", default = "redis://127.0.0.1:6379")]
    pub redis: String,

    #[envconfig(from = "DUO_API", default = "1")]
    pub duo_api: String,

    #[envconfig(from = "SPOTIFY_CLIENT_ID", default = "")]
    pub spotify_client_id: String,

    #[envconfig(from = "SPOTIFY_CLIENT_SECRET", default = "")]
    pub spotify_client_secret: String,

    #[envconfig(from = "GITHUB_SECRET", default = "")]
    pub github_secret: String,

    #[envconfig(
        from = "SPOTIFY_REDIRECT_URI",
        default = "http://127.0.0.1:8080/v1/spotify/callback"
    )]
    pub spotify_redirect_uri: String,
}
