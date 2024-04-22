use crate::error::RastraError;
use crate::network::conn_info::ConnInfo;
use crate::network::connection::connection::{ConnectionReadHalf, ConnectionWriteHalf};
use crate::network::packet_info::GamePacket;
use crate::network::packet_io::packet_io_writer::PacketWriter;
use crate::network::utils::handler::encode;
use crate::utils::play_status::PlayStatus;

pub async fn handle_play_status(connection_read_half: &mut ConnectionReadHalf, connection_write_half: &ConnectionWriteHalf, conn_info: &mut ConnInfo, play_status: PlayStatus) -> Result<(), RastraError> {
    let mut writer = PacketWriter::new_game_packet_writer();
    writer.write_i32(play_status as i32);

    let data = encode(vec![writer.get_payload(GamePacket::PlayStatus)], conn_info).await.unwrap();

    connection_write_half.send(&data, true).await.unwrap();
    
    Ok(())
}