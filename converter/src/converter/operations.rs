use crate::converter_service::{
    GifRequest, GifResponse, Mp4Request, Mp4Response, PngRequest, PngResponse, WebpRequest,
    WebpResponse,
};
use tonic::Request;

impl crate::service::ConverterServiceImpl {
    pub(crate) fn convert_gif_internal(
        &self,
        _request: Request<GifRequest>,
    ) -> Result<GifResponse, Box<dyn std::error::Error>> {
        Ok(GifResponse {
            content: "converted to gif".as_bytes().to_vec(),
        })
    }

    pub(crate) fn convert_webp_internal(
        &self,
        _request: Request<WebpRequest>,
    ) -> Result<WebpResponse, Box<dyn std::error::Error>> {
        Ok(WebpResponse {
            content: "converted to webp".as_bytes().to_vec(),
        })
    }

    pub(crate) fn convert_mp4_internal(
        &self,
        _request: Request<Mp4Request>,
    ) -> Result<Mp4Response, Box<dyn std::error::Error>> {
        Ok(Mp4Response {
            content: "converted to mp4".as_bytes().to_vec(),
        })
    }

    pub(crate) fn convert_png_internal(
        &self,
        _request: Request<PngRequest>,
    ) -> Result<PngResponse, Box<dyn std::error::Error>> {
        Ok(PngResponse {
            content: "converted to png".as_bytes().to_vec(),
        })
    }
}
