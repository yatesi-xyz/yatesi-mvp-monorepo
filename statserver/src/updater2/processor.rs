use futures::TryFutureExt;
use thiserror::Error as ThisError;

use crate::{
    config::{CacheConfig, DatabaseConfig},
    updater2::{cache::CacheManager, database::DatabaseManager},
};

use super::{cache, database};

#[derive(ThisError, Debug)]
pub enum ProcessorError {
    #[error("failed to perform database operation: {0:?}")]
    Database(#[from] database::DatabaseError),

    #[error("failed to perform cache operation: {0:?}")]
    Cache(#[from] cache::CacheError),

    #[error("failed to join multiple async tasks: {0:?}")]
    AsyncJoin(#[from] tokio::task::JoinError),
}

pub struct LiveUpdateProcessor {
    database: DatabaseManager,
    cache: CacheManager,
}

impl LiveUpdateProcessor {
    pub async fn new(
        database_config: DatabaseConfig,
        cache_config: CacheConfig,
    ) -> Result<LiveUpdateProcessor, ProcessorError> {
        let (database, cache) = tokio::try_join!(
            tokio::spawn(DatabaseManager::new(database_config).map_err(ProcessorError::Database)),
            tokio::spawn(CacheManager::new(cache_config).map_err(ProcessorError::Cache))
        )
        .map_err(ProcessorError::AsyncJoin)?;

        Ok(LiveUpdateProcessor {
            database: database?,
            cache: cache?,
        })
    }

    pub async fn serve(&self) -> Result<(), ProcessorError> {
        todo!("implement service");

        Ok(())
    }
}
