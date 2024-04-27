use bedrock_core::types::*;
use serialize_derive::{MCDeserialize, MCSerialize};

use serialize::de::MCDeserialize;
use serialize::ser::MCSerialize;

use crate::types::connection_request::ConnectionRequestType;

#[derive(Debug, MCSerialize, MCDeserialize)]
pub struct LoginPacket {
    pub client_network_version: i32be,
    pub connection_request: ConnectionRequestType,
}
