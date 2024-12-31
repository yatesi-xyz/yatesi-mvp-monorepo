use futures::TryFutureExt;
use redis::aio::{ConnectionManager, ConnectionManagerConfig};
use thiserror::Error as ThisError;

use crate::config::CacheConfig;

#[derive(ThisError, Debug)]
pub enum CacheError {
    #[error("failed to open connection to cache service: {0:?}")]
    Connection(redis::RedisError),

    #[error("failed to execute cache service command: {0:?}")]
    Command(redis::RedisError),
}

pub struct CacheManager {
    pool: ConnectionManager,
}

impl CacheManager {
    pub async fn new(config: CacheConfig) -> Result<CacheManager, CacheError> {
        let client = redis::Client::open(config.dsn.as_str()).map_err(CacheError::Connection)?;

        let connection_manager = client
            .get_connection_manager_with_config(CacheManager::build_connection_config(config))
            .map_err(CacheError::Connection)
            .await?;

        let _: () = redis::cmd("ping")
            .query_async(&mut connection_manager.clone())
            .map_err(CacheError::Command)
            .await?;

        Ok(CacheManager {
            pool: connection_manager,
        })
    }

    pub async fn set<K, V>(&self, key: K, value: V) -> Result<(), CacheError>
    where
        K: redis::ToRedisArgs,
        V: redis::ToRedisArgs,
    {
        redis::cmd("set")
            .arg(&key)
            .arg(&value)
            .query_async(&mut self.pool.clone())
            .map_err(CacheError::Command)
            .await
    }

    pub async fn get<K, V>(&self, key: K) -> Result<V, CacheError>
    where
        K: redis::ToRedisArgs,
        V: redis::FromRedisValue,
    {
        redis::cmd("get")
            .arg(&key)
            .query_async(&mut self.pool.clone())
            .map_err(CacheError::Command)
            .await
    }

    fn build_connection_config(config: CacheConfig) -> ConnectionManagerConfig {
        ConnectionManagerConfig::new()
            .set_connection_timeout(config.connection_timeout)
            .set_response_timeout(config.response_timeout)
            .set_number_of_retries(config.number_of_retries)
            .set_max_delay(config.max_delay_between_retries.as_millis() as u64)
            .set_exponent_base(config.delay_exponent_base.as_millis() as u64)
    }
}
