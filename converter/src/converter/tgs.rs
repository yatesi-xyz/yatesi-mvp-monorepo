use flate2::read::GzDecoder;
use lottieconv::{Animation, Converter, Rgba};
use prost::bytes::Buf;
use std::error::Error;
use std::io::prelude::*;

pub struct TGSAnimation {
    lottie: Animation,
}

#[derive(Debug)]
struct LottieLoadError {}

impl std::fmt::Display for LottieLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to parse lottie animation")
    }
}
impl Error for LottieLoadError {}

impl TGSAnimation {
    pub fn load_from_tgs_bytes(bytes: Vec<u8>) -> Result<TGSAnimation, Box<dyn Error>> {
        Ok(bytes)
            .and_then(TGSAnimation::decompress_if_gzip)
            .and_then(|content| {
                Animation::from_data(content, "", "").ok_or(LottieLoadError {}.into())
            })
            .and_then(|animation| Ok(TGSAnimation { lottie: animation }))
    }

    pub fn export_to_gif_bytes(self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut buffer = Vec::<u8>::new();
        let _ = Converter::new(self.lottie)
            .gif(Rgba::new_alpha(0, 0, 0, true), &mut buffer)?
            .convert()?;

        Ok(buffer)
    }

    pub fn export_to_webp_bytes(self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut buffer = Vec::new();
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
