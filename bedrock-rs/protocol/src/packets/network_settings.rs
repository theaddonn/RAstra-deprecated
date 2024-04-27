use bedrock_core::types::*;
use serialize_derive::{MCDeserialize, MCSerialize};

#[derive(Debug, Copy, Clone, MCSerialize, MCDeserialize)]
pub struct NetworkSettingsPacket {
    /// Determines the smallest size of raw network payload to compress.
    /// - 0 is disable compression
    /// - 1 is compress everything 1 byte or larger (so everything)
    pub compression_threshold: u16le,
    pub compression_algorythm: u16le,
    pub client_throttle_enabled: bool,
    pub client_throttle_threshold: u8,
    pub client_throttle_scalar: f32,
}
