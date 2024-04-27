use crate::info::GamePacketType;

pub fn encode_gamepacket(data: &Vec<u8>, game_packet_type: GamePacketType) -> Vec<u8> {
    vec![]
}

pub fn decode_gamepacket(data: &Vec<u8>) -> (Vec<u8>, GamePacketType) {
    (vec![], GamePacketType::NetworkSettings)
}
