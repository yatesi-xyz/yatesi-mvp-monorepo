mod config;
mod updater;

use anyhow::{Context, Result as AnyResult};
use config::Config;
use futures::FutureExt;
use updater::LiveUpdateProcessor;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> AnyResult<()> {
    let config: Config = std::fs::read_to_string("config.toml")
        .context("reading config file")
        .and_then(|s| toml::from_str(&s).map_err(|e| e.into()))
        .context("parsing config file")?;

    let live_updates: LiveUpdateProcessor = LiveUpdateProcessor::new(&config.database, &config.cache)
        .map(|r| r.context("initialising batch processor"))
        .await?;

    live_updates
        .listen()
        .map(|r| r.context("listening and batching updates"))
        .await?;

    Ok(tokio::signal::ctrl_c().await?)
}
