use tokio::time::Duration;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub debug: bool,
    pub database: DatabaseConfig,
    pub cache: CacheConfig,
    pub server: ServerConfig,
    pub burst: BurstConfig,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub dsn: String,
    pub username: String,
    pub password: String,
    pub namespace: String,
    pub database: String,
}

#[derive(Debug, Deserialize)]
pub struct CacheConfig {
    pub dsn: String,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct BurstConfig {
    #[serde(with = "humantime_serde")]
    pub sync_interval: Duration,

    #[serde(with = "humantime_serde")]
    pub expire: Duration,
}
