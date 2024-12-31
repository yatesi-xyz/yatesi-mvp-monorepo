use tokio::time::Duration;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub debug: bool,
    pub database: DatabaseConfig,
    pub cache: CacheConfig,
    pub server: ServerConfig,
    pub burst: BurstConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub dsn: String,
    pub username: String,
    pub password: String,
    pub namespace: String,
    pub database: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CacheConfig {
    pub dsn: String,
    #[serde(with = "humantime_serde")]
    pub sync_interval: Duration,
    #[serde(with = "humantime_serde")]
    pub connection_timeout: Duration,
    #[serde(with = "humantime_serde")]
    pub response_timeout: Duration,
    pub number_of_retries: usize,
    #[serde(with = "humantime_serde")]
    pub max_delay_between_retries: Duration,
    #[serde(with = "humantime_serde")]
    pub delay_exponent_base: Duration,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BurstConfig {
    #[serde(with = "humantime_serde")]
    pub sync_interval: Duration,

    #[serde(with = "humantime_serde")]
    pub expire: Duration,
}
