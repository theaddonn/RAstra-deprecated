use bytes::Buf;
use varint_rs::VarintReader;

use crate::{log_info, log_warning};
use crate::network::gamepacket::GamepacketClient;
use crate::network::packet::packet_request_network_settings::PacketRequestNetworkSettings;
use crate::network::packet_handlers::packet_handler_request_network_settings::handle_request_network_settings;
use crate::network::packet_info::GamePacket;

pub async fn handle_packet(packet: Vec<u8>, peer_id: u64) {
    match packet.reader().read_u64_varint() {
        Ok(packet_id) => match packet_id {
            0xC1 => {
                let packet = PacketRequestNetworkSettings::deserialize(packet).unwrap();

                log_info!("RequestNetworkSettingsPacket", format!("{:?}", packet));

                tokio::spawn(handle_request_network_settings(packet, peer_id));
            }
            _ => {
                log_warning!(format!(
                    "GOT UNKNOWN GAMEPACKET PACKET: {:?}",
                    GamePacket::from_id(packet_id).unwrap()
                ));
                loop {}
            }
        },
        Err(_) => {
            log_warning!("PACKET WAS INVALID!")
        }
    }
}
