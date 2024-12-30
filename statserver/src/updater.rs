use crate::config::{CacheConfig, DatabaseConfig};
use anyhow::{Context, Result as AnyResult};
use futures::task::Poll;
use futures::{stream, FutureExt, StreamExt};
use redis::aio::{ConnectionLike, ConnectionManager};
use serde::Deserialize;
use std::future::{Future, IntoFuture};
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    method::Stream,
    opt::auth::Root,
    Surreal,
};

pub struct LiveUpdateProcessor {
    cache: redis::Client,
    database: Surreal<Client>,
}

#[derive(Debug, Deserialize)]
struct CountUpdate {
    count: usize,
}

impl LiveUpdateProcessor {
    pub async fn new(database_cfg: &DatabaseConfig, cache_cfg: &CacheConfig) -> AnyResult<LiveUpdateProcessor> {
        Ok(LiveUpdateProcessor {
            cache: LiveUpdateProcessor::init_cache(cache_cfg).await?,
            database: LiveUpdateProcessor::init_database(database_cfg).await?,
        })
    }

    pub async fn listen(&self) -> AnyResult<()> {
        let mux_connection = self.get_cache_connection().await?;

        let _results = tokio::join!(
            self.handle_stream(mux_connection.clone(), "total_emoji_count"),
            self.handle_stream(mux_connection.clone(), "total_emojipack_count"),
            self.handle_stream(mux_connection.clone(), "indexed_emoji_count"),
            self.handle_stream(mux_connection.clone(), "indexed_emojipack_count"),
        );

        Ok(())
    }

    async fn init_database(database_cfg: &DatabaseConfig) -> AnyResult<Surreal<Client>> {
        let database = Surreal::<Client>::init();

        database
            .connect::<Ws>(&database_cfg.dsn)
            .into_future()
            .map(|r| r.context("connecting to surrealdb on specified address"))
            .await?;

        database
            .signin(Root {
                username: &database_cfg.username,
                password: &database_cfg.password,
            })
            .into_future()
            .map(|r| r.context("authorizing in surrealdb with specified credentials"))
            .await?;

        database
            .use_ns(&database_cfg.namespace)
            .use_db(&database_cfg.database)
            .into_future()
            .map(|r| r.context("switching to specified namespace and database"))
            .await?;

        Ok(database)
    }

    async fn init_cache(cache_cfg: &CacheConfig) -> AnyResult<redis::Client> {
        let cache = redis::Client::open(cache_cfg.dsn.as_str())?;

        let mut connection = cache
            .get_multiplexed_tokio_connection()
            .map(|r| r.context("openning new connection to redis"))
            .await?;

        let _: () = redis::cmd("ping")
            .query_async(&mut connection)
            .map(|r| r.context("executing ping command to verify connection"))
            .await?;

        Ok(cache)
    }

    async fn handle_stream<C: ConnectionLike + Clone>(&self, cache: C, resource: &str) -> AnyResult<()> {
        let mut stream: Stream<Vec<CountUpdate>> = self
            .database
            .select(resource)
            .live()
            .into_future()
            .map(|r| r.context(format!("binding live select to {} resource", &resource)))
            .await?;

        while let Some(notification) = stream.next().await {
            match notification {
                Err(_) => continue,
                Ok(update) => {
                    let _: AnyResult<()> = redis::cmd("set")
                        .arg(&resource)
                        .arg(update.data.count)
                        .query_async(&mut cache.clone())
                        .into_future()
                        .map(|r| r.context("updating cache value"))
                        .await;
                }
            }
        }

        Ok(())
    }

    fn get_cache_connection(&self) -> impl Future<Output = AnyResult<ConnectionManager>> + '_ {
        self.cache
            .get_connection_manager()
            .map(|r| r.context("openning new connection to keydb"))
    }
}
