use std::error::Error;

use crate::{
    converter::tgs::TGSAnimation,
    converter_service::{GifRequest, GifResponse, WebpRequest, WebpResponse},
};
use tonic::Request;

impl crate::service::ConverterServiceImpl {
    pub(crate) fn convert_gif_internal(&self, request: Request<GifRequest>) -> Result<GifResponse, Box<dyn Error>> {
        Ok(request.into_inner().content)
            .and_then(TGSAnimation::load_from_tgs_bytes)
            .and_then(|tgs| tgs.export_to_gif_bytes())
            .and_then(|content| Ok(GifResponse { content }))
    }

    pub(crate) fn convert_webp_internal(&self, request: Request<WebpRequest>) -> Result<WebpResponse, Box<dyn Error>> {
        Ok(request.into_inner().content)
            .and_then(TGSAnimation::load_from_tgs_bytes)
            .and_then(|tgs| tgs.export_to_webp_bytes())
            .and_then(|content| Ok(WebpResponse { content }))
    }
}
