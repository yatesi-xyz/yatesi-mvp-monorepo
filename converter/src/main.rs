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

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("converter_service_descriptor");
}

use converter_service::converter_service_server::ConverterServiceServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let config: Config =
        toml::from_str(&fs::read_to_string("config.toml").expect("Failed to read config file"))
            .expect("Failed to parse config file");

    let addr = format!("{}:{}", config.server.host, config.server.port).parse()?;
    let converter_service = ConverterServiceServer::new(ConverterServiceImpl::default());
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(converter_service::FILE_DESCRIPTOR_SET)
        .build_v1()?;

    info!("ConverterService listening on {}", addr);

    Server::builder()
        .add_service(reflection_service)
        .add_service(converter_service)
        .serve(addr)
        .await?;

    Ok(())
}
