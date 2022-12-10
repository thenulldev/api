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

    #[envconfig(from = "DUO_API")]
    pub duo_api: String,

    #[envconfig(from = "SPOTIFY_CLIENT_ID")]
    pub spotify_client_id: String,

    #[envconfig(from = "SPOTIFY_CLIENT_SECRET")]
    pub spotify_client_secret: String,

    #[envconfig(from = "GITHUB_SECRET")]
    pub github_secret: String,

    #[envconfig(from = "SPOTIFY_REDIRECT_URI")]
    pub spotify_redirect_uri: String,
}
