use crate::error::RastraError;
use crate::network::conn_info::ConnInfo;
use crate::network::connection::connection::ConnectionWriteHalf;
use crate::network::packet_info::GamePacket;
use crate::network::packet_io::packet_io_writer::PacketWriter;
use crate::network::utils::handler::encode;

pub async fn handle_packs(
    connection_write_half: &ConnectionWriteHalf,
    conn_info: &mut ConnInfo,
) -> Result<(), RastraError> {
    let mut writer = PacketWriter::new_game_packet_writer();

    writer.write_bool(false);
    writer.write_bool(false);
    writer.write_bool(false);
    writer.write_bool(false);

    writer.write_i16(0);

    writer.write_i16(0);

    writer.write_u32_varint(0);

    let data = encode(
        vec![writer.get_payload(GamePacket::ResourcePacksInfo)],
        conn_info,
    )
    .await
    .unwrap();

    connection_write_half.send(&data, true).await.unwrap();

    let mut writer = PacketWriter::new_game_packet_writer();

    writer.write_bool(false);

    writer.write_u32_varint(0);

    writer.write_u32_varint(0);

    writer.write_string("1.20.72".to_string());

    writer.write_u32(0);
    writer.write_bool(false);

    let data = encode(
        vec![writer.get_payload(GamePacket::ResourcePackStack)],
        conn_info,
    )
    .await
    .unwrap();

    connection_write_half.send(&data, true).await.unwrap();

    Ok(())
}
