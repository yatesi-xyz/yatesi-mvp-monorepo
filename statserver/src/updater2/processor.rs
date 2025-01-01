use futures::TryFutureExt;
use thiserror::Error as ThisError;

use crate::{
    config::{CacheConfig, DatabaseConfig},
    updater2::{cache::CacheManager, database::DatabaseManager},
};

use super::{cache, database, statistics::Statistics};

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

    pub async fn serve(self: Self) -> Result<(), ProcessorError> {
        let stats = self.load_statistics().await?;
        dbg!(stats);

        Ok(())
    }

    async fn load_statistics(self: Self) -> Result<Statistics, ProcessorError> {
        // fetch values from database in parallel
        let values = tokio::try_join!(
            tokio::spawn({
                let db = self.database.clone();
                async move { db.get_statistic().map_err(ProcessorError::Database).await }
            }),
            tokio::spawn({
                let db = self.database.clone();
                async move { db.get_statistic().map_err(ProcessorError::Database).await }
            }),
            tokio::spawn({
                let db = self.database.clone();
                async move { db.get_statistic().map_err(ProcessorError::Database).await }
            }),
            tokio::spawn({
                let db = self.database.clone();
                async move { db.get_statistic().map_err(ProcessorError::Database).await }
            }),
        )
        .map_err(ProcessorError::AsyncJoin)?;

        Ok(Statistics {
            total_emoji_count: values.0?,
            total_emojipack_count: values.1?,
            indexed_emoji_count: values.2?,
            indexed_emojipack_count: values.3?,
        })
    }
}
