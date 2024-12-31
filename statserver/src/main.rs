mod config;
mod updater;
mod updater2;
mod websocket;

use anyhow::{Context, Result as AnyResult};
use config::Config;
use futures::FutureExt;

use updater::LiveUpdateProcessor;
use websocket::WebsocketServer;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> AnyResult<()> {
    log::debug!("loading config from file");
    let config: Config = std::fs::read_to_string("config.toml")
        .context("reading config file")
        .and_then(|s| toml::from_str(&s).map_err(|e| e.into()))
        .context("parsing config file")?;

    std::env::set_var("RUST_LOG", if config.debug { "DEBUG" } else { "INFO" });
    pretty_env_logger::init();

    log::debug!("initialising live updates processor");
    let live_updates = LiveUpdateProcessor::new(config.database.clone(), config.cache.clone(), config.burst.clone())
        .map(|r| r.context("initialising live updates processor"))
        .await?;

    log::debug!("initialising websocket server");
    let websocket_server = WebsocketServer::new(config.server.clone(), config.cache.clone())
        .map(|r| r.context("initialising websocker server"))
        .await?;

    log::info!("listening for live updates");
    let updates_task = tokio::spawn(async move {
        live_updates
            .listen()
            .map(|r| r.context("listening and batching updates"))
            .await
    });

    log::info!("serving websocker connections");
    let websocket_task = tokio::spawn(async move {
        websocket_server
            .listen()
            .map(|r| r.context("listening and serving websockers"))
            .await
    });

    let results = tokio::join!(updates_task, websocket_task);

    (results.0.expect("failed to join task")).and(results.1.expect("failed to join task"))
}
