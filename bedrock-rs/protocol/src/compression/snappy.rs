use crate::compression::CompressionMethod;
use crate::error::{CompressionError, DecompressionError};

pub struct SnappyCompression {
    threshold: u16,
}

impl CompressionMethod for SnappyCompression {
    fn get_IDu8(&self) -> u8 {
        0x01
    }

    fn get_IDu16(&self) -> u16 {
        0x0001
    }

    fn get_threshold(&self) -> u16 {
        self.threshold
    }

    fn compress(&self, data: &Vec<u8>) -> Result<Vec<u8>, CompressionError> {
        let mut encoder = snap::raw::Encoder::new();

        match encoder.compress_vec(&*data) {
            Ok(v) => Ok(v),
            Err(e) => Err(CompressionError::SnappyError(e)),
        }
    }

    fn decompress(&self, data: &Vec<u8>) -> Result<Vec<u8>, DecompressionError> {
        let mut decoder = snap::raw::Decoder::new();

        match decoder.decompress_vec(&*data) {
            Ok(v) => Ok(v),
            Err(e) => Err(DecompressionError::SnappyError(e)),
        }
    }
}
