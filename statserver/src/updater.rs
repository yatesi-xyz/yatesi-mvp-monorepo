use crate::config::{BurstConfig, CacheConfig, DatabaseConfig};
use anyhow::{Context, Result as AnyResult};
use futures::{stream, FutureExt, StreamExt};
use redis::aio::ConnectionLike;
use serde::Deserialize;
use std::future::IntoFuture;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};
use tokio::time::Instant;

pub struct LiveUpdateProcessor {
    cache: redis::Client,
    database: Surreal<Client>,
    burst_cfg: BurstConfig,
}

#[derive(Debug, Deserialize)]
struct CountUpdate {
    count: usize,
}

enum UpdateEvent {
    TimerTick,
    Database(surrealdb::Notification<CountUpdate>),
}

impl LiveUpdateProcessor {
    pub async fn new(
        database_cfg: DatabaseConfig,
        cache_cfg: CacheConfig,
        burst_cfg: BurstConfig,
    ) -> AnyResult<LiveUpdateProcessor> {
        Ok(LiveUpdateProcessor {
            burst_cfg,
            cache: LiveUpdateProcessor::init_cache(cache_cfg).await?,
            database: LiveUpdateProcessor::init_database(database_cfg).await?,
        })
    }

    pub async fn listen(&self) -> AnyResult<()> {
        let mux_connection = self
            .cache
            .get_connection_manager()
            .map(|r| r.context("openning new connection to keydb"))
            .await?;

        let results = tokio::join!(
            self.handle_stream(mux_connection.clone(), "total_emoji_count"),
            self.handle_stream(mux_connection.clone(), "total_emojipack_count"),
            self.handle_stream(mux_connection.clone(), "indexed_emoji_count"),
            self.handle_stream(mux_connection.clone(), "indexed_emojipack_count"),
        );

        (results.0).and(results.1).and(results.2).and(results.3)
    }

    async fn init_database(database_cfg: DatabaseConfig) -> AnyResult<Surreal<Client>> {
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

    async fn init_cache(cache_cfg: CacheConfig) -> AnyResult<redis::Client> {
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
        let database_stream = self
            .database
            .select(resource)
            .live()
            .into_future()
            .map(|r| r.context(format!("binding live select to {} resource", &resource)))
            .await?
            .map(|r| r.map(|n| UpdateEvent::Database(n)));

        log::debug!(target: resource, "launching timer event stream");
        let timer_stream = Box::pin(stream::unfold((), |_| async {
            tokio::time::sleep(self.burst_cfg.sync_interval).await;
            Some((Ok(UpdateEvent::TimerTick), ()))
        }));

        log::debug!(target: resource, "fetching initial row count from resource");
        let mut last_update = Instant::now();
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
        let mut lastest_count = current_count;

        let mut event_stream = stream::select(timer_stream, database_stream);
        log::debug!(target: resource, "consuming event stream");
        while let Some(event) = event_stream.next().await {
            match event {
                Err(err) => {
                    log::error!(target: resource, "error encounted while consuming stream for resource: {}", err);
                    return Err(err.into());
                }

                Ok(UpdateEvent::Database(update)) => {
                    let now = Instant::now();

                    if update.data.count > current_count {
                        log::debug!(target: resource, "new value {} is greater than current burst counter {}, updating", &update.data.count, &current_count);
                        current_count = update.data.count
                    } else if now.duration_since(last_update) >= self.burst_cfg.expire {
                        log::debug!(target: resource, "new value {} is lower than current burst counter, however, burst has expired", &update.data.count);
                        current_count = update.data.count
                    } else {
                        log::debug!(target: resource, "new value {} is lower than current burst counter, burst is active, skipping update", &update.data.count);
                    }

                    last_update = now;
                }

                Ok(UpdateEvent::TimerTick) => {
                    if current_count == lastest_count {
                        continue;
                    }

                    log::info!(target: resource, "updating remote cache with value: {}", &current_count);
                    let _ = redis::cmd("set")
                        .arg(&resource)
                        .arg(&current_count)
                        .query_async(&mut cache.clone())
                        .into_future()
                        .map(|r| r.context("updating cache value"))
                        .await
                        .map(|_: ()| lastest_count = current_count);
                }
            }
        }

        Ok(())
    }
}
