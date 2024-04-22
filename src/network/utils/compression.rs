use std::io::Read;

use flate2::Compression;

use crate::error::RastraError;
use crate::utils::compression_method::CompressionMethod;

const DEFAULT_FLATE_COMPRESSION_LEVEL: Compression = Compression::best(); // TODO make this adjustable in rastra.toml

pub fn decompress(
    data: Vec<u8>,
    compression_method: &CompressionMethod,
) -> Result<Vec<u8>, RastraError> {
    match compression_method {
        CompressionMethod::Flate => decompress_flate(data),
        CompressionMethod::Snappy => decompress_snappy(data),
    }
}

pub fn compress(
    data: Vec<u8>,
    compression_method: &CompressionMethod,
) -> Result<Vec<u8>, RastraError> {
    match compression_method {
        CompressionMethod::Flate => compress_flate(data),
        CompressionMethod::Snappy => compress_snappy(data),
    }
}

fn decompress_snappy(data: Vec<u8>) -> Result<Vec<u8>, RastraError> {
    let mut decoder = snap::raw::Decoder::new();

    match decoder.decompress_vec(&*data) {
        Ok(v) => Ok(v),
        Err(_) => Err(RastraError::DecompressionError),
    }
}

fn decompress_flate(data: Vec<u8>) -> Result<Vec<u8>, RastraError> {
    let mut decoder = flate2::read::DeflateDecoder::new(&*data);

    let mut buf = vec![];

    match decoder.read_to_end(&mut buf) {
        Ok(_) => Ok(buf),
        Err(_) => Err(RastraError::DecompressionError),
    }
}

fn compress_snappy(data: Vec<u8>) -> Result<Vec<u8>, RastraError> {
    let mut encoder = snap::raw::Encoder::new();

    match encoder.compress_vec(&*data) {
        Ok(v) => Ok(v),
        Err(_) => Err(RastraError::CompressionError),
    }
}

fn compress_flate(data: Vec<u8>) -> Result<Vec<u8>, RastraError> {
    let mut encoder = flate2::read::DeflateEncoder::new(&*data, DEFAULT_FLATE_COMPRESSION_LEVEL);

    let mut buf = vec![];

    match encoder.read_to_end(&mut buf) {
        Ok(_) => Ok(buf),
        Err(_) => Err(RastraError::CompressionError),
    }
}
