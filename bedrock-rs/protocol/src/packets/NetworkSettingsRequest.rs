use serialize_derive::{MCDeserialize, MCSerialize};
use bedrock_core::types::i32be;

use serialize::ser;

#[derive(Copy, Clone, MCSerialize, MCDeserialize)]
pub struct NetworkSettingsRequest {
    client_network_version: i32be,
}