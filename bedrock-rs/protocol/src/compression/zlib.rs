use flate2::Compression;

use crate::compression::CompressionMethod;
use crate::error::{CompressionError, DecompressionError};

pub struct ZlibCompression {
    threshold: u16,
    /// Needs to be a number between 0 and 9.
    /// Indicitaes how compressed the data becomes.
    ///
    /// - 0 = None
    /// - 1 = Fastest
    /// - 6 = Default
    /// - 9 = Best
    compression_level: u8,
}

impl CompressionMethod for ZlibCompression {
    fn get_IDu8(&self) -> u8 {
        0x00
    }

    fn get_IDu16(&self) -> u16 {
        0x0000
    }

    fn get_threshold(&self) -> u16 {
        self.threshold
    }

    fn compress(&self, data: &Vec<u8>) -> Result<Vec<u8>, CompressionError> {
        // Packets encoded by MCBE don't have zlib headers
        let encoder = flate2::Compress::new(Compression::new(self.compression_level as u32), false);
        todo!()
    }

    fn decompress(&self, data: &Vec<u8>) -> Result<Vec<u8>, DecompressionError> {
        todo!()
    }
}
