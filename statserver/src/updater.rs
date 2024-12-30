use crate::config::{CacheConfig, DatabaseConfig};
use anyhow::{Context, Result as AnyResult};
use futures::{FutureExt, StreamExt};
use redis::aio::{ConnectionLike, ConnectionManager};
use serde::Deserialize;
use std::future::{Future, IntoFuture};
use std::time::Duration;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    method::Stream,
    opt::auth::Root,
    Surreal,
};
use tokio::time::Instant;

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
        log::debug!("creating database client");
        let database = Surreal::<Client>::init();

        log::debug!("connecting to websocket");
        database
            .connect::<Ws>(&database_cfg.dsn)
            .into_future()
            .map(|r| r.context("connecting to surrealdb on specified address"))
            .await?;

        log::debug!("authorizing with root username and password");
        database
            .signin(Root {
                username: &database_cfg.username,
                password: &database_cfg.password,
            })
            .into_future()
            .map(|r| r.context("authorizing in surrealdb with specified credentials"))
            .await?;

        log::debug!("switching to specified namespace and database");
        database
            .use_ns(&database_cfg.namespace)
            .use_db(&database_cfg.database)
            .into_future()
            .map(|r| r.context("switching to specified namespace and database"))
            .await?;

        log::info!("database is OK");
        Ok(database)
    }

    async fn init_cache(cache_cfg: &CacheConfig) -> AnyResult<redis::Client> {
        log::debug!("creating cache client");
        let cache = redis::Client::open(cache_cfg.dsn.as_str())?;

        log::debug!("opening new cache connection");
        let mut connection = cache
            .get_multiplexed_tokio_connection()
            .map(|r| r.context("opening new connection to redis"))
            .await?;

        log::debug!("executing ping command");
        let _: () = redis::cmd("ping")
            .query_async(&mut connection)
            .map(|r| r.context("executing ping command to verify connection"))
            .await?;

        log::info!("cache is OK");
        Ok(cache)
    }

    async fn handle_stream<C: ConnectionLike + Clone>(&self, cache: C, resource: &str) -> AnyResult<()> {
        log::debug!(target: resource, "openinig live select stream for resource");
        let mut stream: Stream<Vec<CountUpdate>> = self
            .database
            .select(resource)
            .live()
            .into_future()
            .map(|r| r.context(format!("binding live select to {} resource", &resource)))
            .await?;

        log::debug!(target: resource, "fetching initial row count from resource");
        let mut current_count = self
            .database
            .select(resource)
            .into_future()
            .map(|r: Result<Vec<CountUpdate>, surrealdb::Error>| {
                r.context(format!("getting intial value from {} resource", &resource))
            })
            .await?
            .get(0)
            .map_or(0, |r| r.count);

        let mut last_update = Instant::now();

        log::debug!(target: resource, "consuming resource stream");
        while let Some(notification) = stream.next().await {
            match notification {
                Err(err) => {
                    log::error!(
                        "error encounted while consuming stream for resource {}: {}",
                        &resource,
                        err
                    );
                }
                Ok(update) => {
                    let now = Instant::now();
                    log::debug!(target: resource, "new update received");

                    if update.data.count > current_count {
                        log::debug!(target: resource, "new value is greater than current in-memory cached value, updating");
                        current_count = update.data.count
                    } else if now.duration_since(last_update) >= Duration::from_micros(2000) {
                        log::debug!(target: resource, "new value is lower than current in-memory cached value, however more then 2000ms have passed, updating");
                        current_count = update.data.count
                    } else {
                        log::debug!(target: resource, "new value is lower than current in-memory cached value, cache has not expiried yet, skipping");
                    }

                    if now.duration_since(last_update) > Duration::from_millis(100) {
                        log::info!(target: resource, "updating remote cache with value: {}", &current_count);
                        let _: AnyResult<()> = redis::cmd("set")
                            .arg(&resource)
                            .arg(&current_count)
                            .query_async(&mut cache.clone())
                            .into_future()
                            .map(|r| r.context("updating cache value"))
                            .await;
                    }

                    last_update = now;
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
