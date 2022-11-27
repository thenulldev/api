use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "LISTEN_HOST", default = "0.0.0.0")]
    pub listen_host: String,

    #[envconfig(from = "LISTEN_PORT", default = "8080")]
    pub listen_port: u16,

    #[envconfig(from = "DB_URL")]
    pub db_url: String,

    #[envconfig(from = "DB_PORT", default = "5432")]
    pub db_port: u16,

    #[envconfig(from = "REDIS_URL", default = "redis://127.0.0.1:6379")]
    pub redis: String,

    #[envconfig(from = "DUO_API")]
    pub duo_api: String,
}
