use crate::config::{CacheConfig, ServerConfig};
use anyhow::{Context, Result as AnyResult};
use futures::{stream, FutureExt, SinkExt, StreamExt};
use redis::aio::{ConnectionLike, ConnectionManagerConfig};
use serde::Serialize;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::Message;

pub struct WebsocketServer {
    server_cfg: ServerConfig,
    cache: redis::Client,
}

#[derive(Clone, Debug, Serialize)]
pub struct StatisticsMessage {
    pub total_emoji_count: usize,
    pub total_emojipack_count: usize,
    pub indexed_emoji_count: usize,
    pub indexed_emojipack_count: usize,
}

enum Event {
    NewMessage(Message),
    TimerTick,
}

impl WebsocketServer {
    pub async fn new(server_cfg: ServerConfig, cache_cfg: CacheConfig) -> AnyResult<WebsocketServer> {
        Ok(WebsocketServer {
            server_cfg,
            cache: WebsocketServer::init_cache(cache_cfg).await?,
        })
    }

    pub async fn listen(&self) -> AnyResult<()> {
        let address = format!("{}:{}", self.server_cfg.host, self.server_cfg.port);

        log::info!("starting tcp listener on {}", &address);
        let listener = TcpListener::bind(address)
            .map(|r| r.context("binding tcp socket"))
            .await?;

        log::debug!("creating connection manager for cache");
        let mux_connection = self
            .cache
            .get_connection_manager_with_config(
                ConnectionManagerConfig::new()
                    .set_max_delay(1000)
                    .set_number_of_retries(5)
                    .set_response_timeout(std::time::Duration::from_millis(500))
                    .set_connection_timeout(std::time::Duration::from_millis(200)),
            )
            .map(|r| r.context("openning new connection to keydb"))
            .await?;

        while let Ok((stream, _)) = listener.accept().await {
            let cache = mux_connection.clone();
            tokio::spawn(async move {
                WebsocketServer::handle_stream(cache, stream)
                    .map(|r| r.context("handling new tcp stream"))
                    .await
                    .map_or_else(
                        |e| {
                            log::error!("websocket stream exit unexpectedly: {}", e);
                        },
                        |_| {
                            log::info!("websocket closed successfully");
                        },
                    )
            });
        }

        Ok(())
    }

    async fn handle_stream<C: ConnectionLike + Clone>(cache: C, stream: TcpStream) -> AnyResult<()> {
        let client_address = stream.peer_addr()?;
        let resource = &format!("websocket [{}]", client_address);
        log::info!(target: resource, "open new websocket for client");

        log::debug!(target: resource, "accepting websocket connection");
        let ws_stream = tokio_tungstenite::accept_async(stream)
            .map(|r| r.context("accepting async websocket stream"))
            .await?;

        let (mut write_stream, read_stream) = ws_stream.split();

        log::debug!(target: resource, "spawning timer stream");
        let timer_stream = Box::pin(stream::unfold((), |_| async {
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            Some((Ok(Event::TimerTick), ()))
        }));
        let message_stream = read_stream.map(|r| r.map(|m| Event::NewMessage(m)));

        let mut event_stream = stream::select(message_stream, timer_stream);
        log::debug!(target: resource, "consuming event stream");
        while let Some(event) = event_stream.next().await {
            match event {
                Err(err) => {
                    log::error!(target: resource, "error encounted while consuming stream: {}", err);
                    break;
                }

                Ok(Event::NewMessage(Message::Ping(b))) => {
                    write_stream
                        .send(Message::Pong(b))
                        .map(|r| r.context("ping-pong websocket reply"))
                        .await?;
                }

                Ok(Event::NewMessage(Message::Close(_))) => {
                    log::info!(target: resource, "recevied closing message");
                    break;
                }

                Ok(Event::TimerTick) | Ok(Event::NewMessage(_)) => {
                    log::debug!(target: resource, "fetching data from cache");
                    let values = tokio::time::timeout(
                        std::time::Duration::from_millis(500),
                        redis::cmd("MGET")
                            .arg(&[
                                "total_emoji_count",
                                "total_emojipack_count",
                                "indexed_emoji_count",
                                "indexed_emojipack_count",
                            ])
                            .query_async(&mut cache.clone()),
                    )
                    .await
                    .inspect_err(|err| log::error!("failed to fetch from cache with timeout: {}", err))
                    .unwrap_or(Ok((0, 0, 0, 0)))
                    .inspect_err(|err| log::error!("failed to fetch from cache with error: {}", err))
                    .unwrap_or((0, 0, 0, 0));

                    log::debug!(target: resource, "generating message");
                    let data = StatisticsMessage {
                        total_emoji_count: values.0,
                        total_emojipack_count: values.1,
                        indexed_emoji_count: values.2,
                        indexed_emojipack_count: values.3,
                    };
                    let message = serde_json::to_string(&data).context("serializing statistics message")?;

                    log::debug!(target: resource, "sending message");
                    write_stream
                        .send(Message::text(message))
                        .map(|r| r.context("writing to output stream"))
                        .await?;
                }
            }
        }

        Ok(())
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
}
