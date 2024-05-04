use crate::compression::none::NoCompression;
use crate::compression::snappy::SnappyCompression;
use crate::compression::zlib::ZlibCompression;
use crate::error::{CompressionError, DecompressionError};

pub mod none;
pub mod zlib;
pub mod snappy;

pub trait CompressionMethod {
    /// Used after the raknet game packet header id for identifying with which
    /// CompressionMethod a packet was compressed
    fn get_IDu8(&self) -> u8;
    /// Used in the NetworkSettingsPacket to identify which
    /// CompressionMethod should be used
    fn get_IDu16(&self) -> u16;
    /// Get the compression threshold of the CompressionMethod.
    fn get_threshold(&self) -> u16;

    /// Compress the given data and return an owned Vector
    /// with the compressed data
    fn compress(&self, data: &Vec<u8>) -> Result<Vec<u8>, CompressionError>;
    /// Decompress the given compressed data and return an owned Vector
    /// with the decompressed data
    fn decompress(&self, data: &Vec<u8>) -> Result<Vec<u8>, DecompressionError>;
}
