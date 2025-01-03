use std::future::IntoFuture;

use futures::TryFutureExt;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};
use thiserror::Error as ThisError;

use crate::config::DatabaseConfig;

use super::statistics::Statistic;

#[derive(ThisError, Debug)]
pub enum DatabaseError {
    #[error("failed to connect to database: {0:?}")]
    Connection(surrealdb::Error),

    #[error("failed to authorize to database: {0:?}")]
    Authorization(surrealdb::Error),

    #[error("failed to switch namespace and database: {0:?}")]
    Namespace(surrealdb::Error),

    #[error("failed to execute query: {0:?}")]
    Command(surrealdb::Error),

    #[error("failed to fetch any rows by query")]
    NoResponse,
}

#[derive(Debug, Clone)]
pub struct DatabaseManager {
    config: DatabaseConfig,
    client: Surreal<Client>,
}

#[derive(Deserialize, Serialize, Debug)]
struct ResourceCount {
    pub count: usize,
}

impl DatabaseManager {
    pub async fn new(config: DatabaseConfig) -> Result<DatabaseManager, DatabaseError> {
        let client = Surreal::<Client>::init();
        let manager = DatabaseManager { config, client };
        manager.connect().await?;

        Ok(manager)
    }

    pub async fn connect(&self) -> Result<(), DatabaseError> {
        self.client
            .connect::<Ws>(&self.config.dsn)
            .into_future()
            .map_err(DatabaseError::Connection)
            .await?;

        self.client
            .signin(Root {
                username: &self.config.username,
                password: &self.config.password,
            })
            .into_future()
            .map_err(DatabaseError::Authorization)
            .await?;

        self.client
            .use_ns(&self.config.namespace)
            .use_db(&self.config.database)
            .into_future()
            .map_err(DatabaseError::Namespace)
            .await?;

        Ok(())
    }

    pub async fn get_statistic<S, T>(&self) -> Result<S, DatabaseError>
    where
        S: Statistic<T>,
        T: DeserializeOwned,
    {
        self.client
            .query(r#"(SELECT $field FROM $source)[0].$field"#)
            .bind(("field", S::FIELD_NAME))
            .bind(("source", S::SOURCE_TABLE))
            .into_future()
            .map_err(DatabaseError::Command)
            .await?
            .take::<Option<T>>(0)
            .map_err(DatabaseError::Command)?
            .map(S::from)
            .ok_or(DatabaseError::NoResponse)
    }
}
