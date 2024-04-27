use bedrock_core::types::*;
use serialize_derive::{MCDeserialize, MCSerialize};

#[derive(Debug, Copy, Clone, MCSerialize, MCDeserialize)]
pub struct NetworkSettingsRequestPacket {
    pub client_network_version: i32be,
}
