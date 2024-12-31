use std::future::IntoFuture;

use futures::TryFutureExt;
use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};
use thiserror::Error as ThisError;

use crate::config::DatabaseConfig;

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

    pub async fn get_resource_value(&self, resource: &str) -> Result<usize, DatabaseError> {
        self.client
            .select(resource)
            .into_future()
            .map_err(DatabaseError::Command)
            .await?
            .get(0)
            .map(|d: &ResourceCount| d.count)
            .ok_or(DatabaseError::NoResponse)
    }
}
