use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
    pub cache: CacheConfig,
    pub server: ServerConfig,
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
