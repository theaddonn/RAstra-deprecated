use serialize_derive::{MCDeserialize, MCSerialize};
use bedrock_core::types::u16le;

#[derive(Copy, Clone, MCSerialize, MCDeserialize)]
pub struct NetworkSettings {
    compression_threashold: u16le,
    compression_algorythm: u16le,
    client_throttle_enabled: bool,
    client_throttle_threshold: u8,
    client_throtle_scalar: f32
}
