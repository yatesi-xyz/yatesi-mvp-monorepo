mod config;
mod converter;
mod service;

use config::Config;
use service::ConverterServiceImpl;
use std::fs;
use tonic::transport::Server;
use tracing::info;

pub mod converter_service {
    tonic::include_proto!("converter_service");
}

use converter_service::converter_service_server::ConverterServiceServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let config: Config =
        toml::from_str(&fs::read_to_string("config.toml").expect("Failed to read config file"))?;

    let addr = format!("{}:{}", config.server.host, config.server.port).parse()?;
    let converter_service = ConverterServiceImpl::default();

    info!("ConverterService listening on {}", addr);

    Server::builder()
        .add_service(ConverterServiceServer::new(converter_service))
        .serve(addr)
        .await?;

    Ok(())
}
