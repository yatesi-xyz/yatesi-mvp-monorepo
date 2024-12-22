use tonic::{Request, Response, Status};
use tracing::{error, info};

use crate::converter_service::converter_service_server::ConverterService;
use crate::converter_service::{
    GifRequest, GifResponse, Mp4Request, Mp4Response, PngRequest, PngResponse, WebpRequest,
    WebpResponse,
};

#[derive(Debug)]
pub struct ConverterServiceImpl {}

impl Default for ConverterServiceImpl {
    fn default() -> Self {
        Self {}
    }
}

#[tonic::async_trait]
impl ConverterService for ConverterServiceImpl {
    async fn convert_to_gif(
        &self,
        request: Request<GifRequest>,
    ) -> Result<Response<GifResponse>, Status> {
        info!("Starting GIF conversion");

        let result = match self.convert_gif_internal(request) {
            Ok(response) => {
                info!("GIF conversion successful");
                Ok(Response::new(response))
            }
            Err(e) => {
                error!("GIF conversion failed: {}", e);
                Err(Status::internal(e.to_string()))
            }
        };

        result
    }

    async fn convert_to_webp(
        &self,
        request: Request<WebpRequest>,
    ) -> Result<Response<WebpResponse>, Status> {
        info!("Starting WebP conversion");

        let result = match self.convert_webp_internal(request) {
            Ok(response) => {
                info!("WebP conversion successful");
                Ok(Response::new(response))
            }
            Err(e) => {
                error!("WebP conversion failed: {}", e);
                Err(Status::internal(e.to_string()))
            }
        };

        result
    }

    async fn convert_to_mp4(
        &self,
        request: Request<Mp4Request>,
    ) -> Result<Response<Mp4Response>, Status> {
        info!("Starting MP4 conversion");

        let result = match self.convert_mp4_internal(request) {
            Ok(response) => {
                info!("MP4 conversion successful");
                Ok(Response::new(response))
            }
            Err(e) => {
                error!("MP4 conversion failed: {}", e);
                Err(Status::internal(e.to_string()))
            }
        };

        result
    }

    async fn convert_to_png(
        &self,
        request: Request<PngRequest>,
    ) -> Result<Response<PngResponse>, Status> {
        info!("Starting PNG conversion");

        let result = match self.convert_png_internal(request) {
            Ok(response) => {
                info!("PNG conversion successful");
                Ok(Response::new(response))
            }
            Err(e) => {
                error!("PNG conversion failed: {}", e);
                Err(Status::internal(e.to_string()))
            }
        };

        result
    }
}
