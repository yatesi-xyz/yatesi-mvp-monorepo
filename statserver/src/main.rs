mod config;

use config::Config;
use futures::StreamExt;
use serde::Deserialize;
use std::error::Error;
use std::sync::LazyLock;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::method::Stream;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

#[derive(Deserialize, Debug)]
struct EmojisCount {
    count: usize,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config: Config = toml::from_str(
        &std::fs::read_to_string("config.toml").expect("failed to read config file"),
    )
    .expect("failed to parse config file");

    DB.connect::<Ws>(config.database.dsn).await?;
    DB.signin(Root {
        username: &config.database.username,
        password: &config.database.password,
    })
    .await?;

    DB.use_ns(&config.database.namespace)
        .use_db(&config.database.database)
        .await?;

    DB.query(
        r#"
        DEFINE TABLE IF NOT EXISTS emojis_count_view TYPE NORMAL AS
        SELECT count() FROM emojis GROUP ALL;
        "#,
    )
    .await?;

    let mut stream: Stream<Vec<EmojisCount>> = DB.select("emojis_count_view").live().await?;

    while let Some(result) = stream.next().await {
        match result {
            Ok(n) => println!("new count: {}", n.data.count),
            Err(_) => break,
        }
    }

    Ok(())
}
