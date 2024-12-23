use std::error::Error;

#[derive(Debug)]
pub struct GzipDecompressionError {}

impl std::fmt::Display for GzipDecompressionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to decompress tgs bytes")
    }
}
impl Error for GzipDecompressionError {}

#[derive(Debug)]
pub struct LottieParseError {}

impl std::fmt::Display for LottieParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to parse lottie animation")
    }
}
impl Error for LottieParseError {}
