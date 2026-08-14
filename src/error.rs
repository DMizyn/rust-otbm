use std::io;
use thiserror::Error;

/// Errors that can occur when working with OTMB files
#[derive(Error, Debug)]
pub enum OtmbError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("Invalid OTMB file format: {0}")]
    InvalidFormat(String),

    #[error("Unsupported OTMB version: {0}")]
    UnsupportedVersion(u32),

    #[error("Invalid map dimensions: {0}")]
    InvalidDimensions(String),

    #[error("Missing required data: {0}")]
    MissingData(String),

    #[error("Decompression error: {0}")]
    DecompressionError(String),
}
