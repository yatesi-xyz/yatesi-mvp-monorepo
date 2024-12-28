use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub dsn: String,
    pub username: String,
    pub password: String,
    pub namespace: String,
    pub database: String,
}
