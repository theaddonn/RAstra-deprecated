use crate::error::RastraError;
use crate::error::RastraError::CouldNotDeserialize;
use crate::network::gamepacket::GamepacketClient;
use crate::network::packet_io::packet_reader::PacketReader;

#[derive(Debug)]
pub struct PacketRequestNetworkSettings {
    pub protocol_version: u32,
}

impl GamepacketClient for PacketRequestNetworkSettings {
    fn deserialize(data: Vec<u8>) -> Result<Self, RastraError> {
        let mut reader = PacketReader::new_game_packet_reader(data);

        reader.read_u64_varint().unwrap();

        let protocol_version = match reader.read_u32() {
            Ok(val) => val,
            Err(_) => return Err(CouldNotDeserialize),
        };

        Ok(Self { protocol_version })
    }
}
