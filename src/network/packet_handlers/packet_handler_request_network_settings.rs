use crate::{log_info, log_warning};
use crate::network::gamepacket::GamepacketServer;
use crate::network::packet::packet_network_settings::PacketNetworkSettings;
use crate::network::packet::packet_request_network_settings::PacketRequestNetworkSettings;
use crate::server::Server;

pub async fn handle_request_network_settings(_packet: PacketRequestNetworkSettings, peer_id: u64) {
    let server = Server::get_instance().await;
    let server = server.lock().await;

    let player = server.find_player(&peer_id);

    let mut player = match player {
        Some(val) => val,
        None => {
            log_warning!("PacketRequestNetworkSettings-Handler", "NOT FOUND PLAYER!");
            log_warning!(
                "PacketRequestNetworkSettings-Handler",
                format!("ID: {peer_id}")
            );
            return;
        }
    };

    if player.connection_write_half.is_some() {
        let packet = PacketNetworkSettings {
            compression_threshold: server.config.compression_threshold,
            compression_method: server.config.compression_method.to_int(),
            client_throttle_enabled: server.config.client_throttle_enabled,
            client_throttle_threshold: server.config.client_throttle_threshold,
            client_throttle_scalar: server.config.client_throttle_scalar,
        };

        log_info!(
            "PacketRequestNetworkSettings-Handler",
            format!("PACKET: {:?}", packet)
        );

        let data = packet.serialize().unwrap();

        log_info!(
            "PacketRequestNetworkSettings-Handler",
            format!("DATA: {:?}", data)
        );

        player
            .connection_write_half
            .as_ref()
            .unwrap()
            .send(&data, true)
            .await
            .unwrap();

        log_info!("PacketRequestNetworkSettings-Handler", "SEND PACKET!");
    }
}
