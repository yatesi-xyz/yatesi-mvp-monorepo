mod config;
mod updater;

use anyhow::{Context, Result as AnyResult};
use config::Config;
use futures::FutureExt;

use updater::LiveUpdateProcessor;

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
    let live_updates: LiveUpdateProcessor = LiveUpdateProcessor::new(&config.database, &config.cache, &config.burst)
        .map(|r| r.context("initialising live updates processor"))
        .await?;

    log::info!("listening for live updates");
    live_updates
        .listen()
        .map(|r| r.context("listening and batching updates"))
        .await?;

    Ok(())
}
