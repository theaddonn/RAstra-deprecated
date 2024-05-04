use crate::compression::{CompressionMethod};
use crate::error::{CompressionError, DecompressionError};

pub struct NoCompression {}

impl CompressionMethod for NoCompression {
    fn get_IDu8(&self) -> u8 {
        0xFF
    }

    fn get_IDu16(&self) -> u16 {
        0xFFFF
    }
    
    fn get_threshold(&self) -> u16 {
        u16::MAX
    }

    fn compress(&self, data: &Vec<u8>) -> Result<Vec<u8>, CompressionError> {
        // don't compress the data becuase there is no compression
        Ok(data.clone())
    }

    fn decompress(&self, data: &Vec<u8>) -> Result<Vec<u8>, DecompressionError> {
        // don't decompress the data becuase there is no compression
        Ok(data.clone())
    }
}