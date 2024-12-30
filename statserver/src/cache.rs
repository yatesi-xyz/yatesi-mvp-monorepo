use std::future::Future;

use anyhow::{Context, Result as AnyResult};
use futures::FutureExt;
use redis::aio::MultiplexedConnection;

use crate::models::YatesiDataStatistics;

impl KeyDBCache {
    pub async fn new(dsn: &str) -> AnyResult<KeyDBCache> {
        Ok(KeyDBCache { client })
    }

    fn get_async_connection(&self) -> impl Future<Output = AnyResult<MultiplexedConnection>> + '_ {
        self.client
            .get_multiplexed_async_connection()
            .map(|r| r.context("openning new connection to redis"))
    }

    async fn get_statistics(&self) -> AnyResult<YatesiDataStatistics> {
        let mut connection = self.get_async_connection().await?;

        let values: (usize, usize, usize, usize) = redis::Cmd::mget(&[
            "total_emoji_count",
            "total_emojipack_count",
            "indexed_emoji_count",
            "indexed_emojipack_count",
        ])
        .query_async(&mut connection)
        .map(|r| r.context("fetching yatesi data statistics fields"))
        .await?;

        Ok(YatesiDataStatistics {
            total_emoji_count: values.0,
            total_emojipack_count: values.1,
            indexed_emoji_count: values.2,
            indexed_emojipack_count: values.3,
        })
    }

    async fn set_statistics(&self, statistics: YatesiDataStatistics) -> AnyResult<()> {
        let mut connection = self.get_async_connection().await?;

        let _: () = redis::Cmd::mset(&[
            ("total_emoji_count", statistics.total_emoji_count),
            ("total_emojipack_count", statistics.total_emojipack_count),
            ("indexed_emoji_count", statistics.indexed_emoji_count),
            ("indexed_emojipack_count", statistics.indexed_emojipack_count),
        ])
        .query_async(&mut connection)
        .map(|r| r.context("updating yatesi data statistics fields"))
        .await?;

        Ok(())
    }
}
