use bytes::Buf;
use varint_rs::VarintReader;

use crate::network::gamepacket::GamepacketClient;
use crate::network::packet::packet_request_network_settings::PacketRequestNetworkSettings;
use crate::{info, warning};

pub async fn handle_packet(packet: Vec<u8>) {
    match packet.reader().read_u64_varint() {
        Ok(packet_id) => match packet_id {
            0xC1 => {
                let packet = PacketRequestNetworkSettings::deserialize(&*packet).unwrap();

                info!(
                    "PacketRequestNetworkSettings",
                    format!("Protocol version: {}", packet.protocol_version)
                )
            }
            _ => {
                warning!("GOT UNKNOWN GAMEPACKET PACKET!")
            }
        },
        Err(_) => {
            warning!("PACKET WAS INVALID!")
        }
    }
}
