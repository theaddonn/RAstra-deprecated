use crate::error::RastraError;
use crate::log_info;
use crate::network::gamepacket::GamepacketClient;

#[derive(Debug)]
struct PacketLogin {
    protocol_version: u32,
    chain_data: Vec<Vec<u8>>,
    skin_data: Vec<u8>,
}

impl GamepacketClient for PacketLogin {
    fn deserialize(data: Vec<u8>) -> Result<Self, RastraError>
    where
        Self: Sized,
    {
        log_info!("PacketLogin", format!("{data:?}"));
        Err(RastraError::DecompressionError)
    }
}
