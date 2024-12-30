use tonic::{Request, Response, Status};
use tracing::{error, info};

use crate::converter_service::converter_service_server::ConverterService;
use crate::converter_service::{GifRequest, GifResponse, WebpRequest, WebpResponse};

#[derive(Debug, Default)]
pub struct ConverterServiceImpl {}

#[tonic::async_trait]
impl ConverterService for ConverterServiceImpl {
    async fn convert_to_gif(&self, request: Request<GifRequest>) -> Result<Response<GifResponse>, Status> {
        info!("Starting GIF conversion");

        match self.convert_gif_internal(request) {
            Ok(response) => {
                info!("GIF conversion successful");
                Ok(Response::new(response))
            }
            Err(e) => {
                error!("GIF conversion failed: {}", e);
                Err(Status::internal(e.to_string()))
            }
        }
    }

    async fn convert_to_webp(&self, request: Request<WebpRequest>) -> Result<Response<WebpResponse>, Status> {
        info!("Starting WebP conversion");

        match self.convert_webp_internal(request) {
            Ok(response) => {
                info!("WebP conversion successful");
                Ok(Response::new(response))
            }

            Err(e) => {
                error!("WebP conversion failed: {}", e);
                Err(Status::internal(e.to_string()))
            }
        }
    }
}
