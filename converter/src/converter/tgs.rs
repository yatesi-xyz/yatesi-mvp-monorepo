use crate::converter::errors::{GzipDecompressionError, LottieParseError};
use flate2::read::GzDecoder;
use lottieconv::{Animation, Converter, Rgba};
use prost::bytes::Buf;
use std::error::Error;
use std::io::prelude::*;

pub struct TGSAnimation {
    lottie: Animation,
}

impl TGSAnimation {
    pub fn load_from_tgs_bytes(bytes: Vec<u8>) -> Result<TGSAnimation, Box<dyn Error>> {
        Ok(bytes)
            .and_then(|bytes| {
                TGSAnimation::decompress_if_gzip(bytes)
                    .map_err(|_| GzipDecompressionError {}.into())
            })
            .and_then(|content| {
                Animation::from_data(content, "", "").ok_or(LottieParseError {}.into())
            })
            .and_then(|animation| Ok(TGSAnimation { lottie: animation }))
    }

    pub fn export_to_gif_bytes(self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut buffer = Vec::with_capacity(
            self.lottie.size().height * self.lottie.size().width * self.lottie.totalframe(),
        );
        let _ = Converter::new(self.lottie)
            .gif(Rgba::new_alpha(0, 0, 0, true), &mut buffer)?
            .convert()?;

        Ok(buffer)
    }

    pub fn export_to_webp_bytes(self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut buffer = Vec::with_capacity(
            self.lottie.size().height * self.lottie.size().width * self.lottie.totalframe(),
        );
        let _ = Converter::new(self.lottie)
            .webp()?
            .convert()?
            .reader()
            .read_to_end(&mut buffer)?;

        Ok(buffer)
    }

    fn decompress_if_gzip(bytes: Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>> {
        if bytes.starts_with(&[0x1f, 0x8b]) {
            let mut buffer = Vec::<u8>::new();
            GzDecoder::new(&bytes[..]).read_to_end(&mut buffer)?;
            Ok(buffer)
        } else {
            Ok(bytes)
        }
    }
}
