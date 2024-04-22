use bytes::BufMut;
use tokio::io::AsyncWriteExt;

use crate::error::RastraError;
use crate::network::gamepacket::GamepacketServer;
use crate::network::packet_info::GamePacket;
use crate::network::packet_io::packet_io_writer::PacketWriter;
use crate::server::Server;

#[derive(Debug)]
pub struct PacketNetworkSettings {
    pub compression_threshold: i16,
    pub compression_method: i16,
    pub client_throttle_enabled: bool,
    pub client_throttle_threshold: u8,
    pub client_throttle_scalar: f32,
}

impl GamepacketServer for PacketNetworkSettings {
    fn serialize(&self) -> Result<Vec<u8>, RastraError> {
        let mut writer = PacketWriter::new_game_packet_writer();

        writer.write_i16(self.compression_threshold);
        writer.write_i16(self.compression_method);
        writer.write_bool(self.client_throttle_enabled);
        writer.write_u8(self.client_throttle_threshold);
        writer.write_f32(self.client_throttle_scalar);

        let mut data = vec![];

        data.put_slice(&*writer.get_payload(GamePacket::NetworkSettings));

        return Ok(data);
    }
}

impl PacketNetworkSettings {
    pub async fn new_from_server() -> Self {
        let server = Server::get_instance().await;
        let server = server.lock().await;

        let config = &server.config;

        // If compression is disabled, set compression_threshold to 0 to disable packet compression
        let compression_threshold = match config.compression_enabled {
            true => { config.compression_threshold },
            false => { 0 }
        };
        
        Self {
            compression_threshold,
            compression_method: config.compression_method.to_int(),
            client_throttle_enabled: config.client_throttle_enabled,
            client_throttle_threshold: config.client_throttle_threshold,
            client_throttle_scalar: config.client_throttle_scalar,
        }
    }
}
