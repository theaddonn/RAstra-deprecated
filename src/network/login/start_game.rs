use crate::error::RastraError;
use crate::network::conn_info::ConnInfo;
use crate::network::connection::connection::ConnectionWriteHalf;
use crate::network::packet_info::GamePacket;
use crate::network::packet_io::packet_io_writer::PacketWriter;
use crate::network::utils::handler::encode;

pub async fn handle_start_game(
    connection_write_half: &ConnectionWriteHalf,
    conn_info: &mut ConnInfo,
) -> Result<(), RastraError> {
    let mut writer = PacketWriter::new_game_packet_writer();

    // Runtime UID & ID
    writer.write_i64_varint(0);
    writer.write_u64_varint(0);

    // Pos
    writer.write_f32(0.0);
    writer.write_f32(0.0);
    writer.write_f32(0.0);

    // Rot
    writer.write_f32(0.0);
    writer.write_f32(0.0);

    // Level settings
    writer.write_u64(0);

    // Spawn settings
    writer.write_i16(0);
    writer.write_string(String::from(""));
    writer.write_i64_varint(0);

    // STILL Level settings
    writer.write_i64_varint(0);
    writer.write_i64_varint(0);
    writer.write_i64_varint(0);

    writer.write_i64_varint(0);
    writer.write_u64_varint(0);
    writer.write_i64_varint(0);

    writer.write_bool(false);
    writer.write_i64_varint(0);
    writer.write_bool(false);
    writer.write_bool(false);
    writer.write_i64_varint(0);
    writer.write_i64_varint(0);
    writer.write_bool(false);
    writer.write_string(String::from(""));
    writer.write_f32(0.0);
    writer.write_f32(0.0);
    writer.write_bool(false);
    writer.write_bool(false);
    writer.write_bool(false);
    writer.write_i64_varint(0);
    writer.write_i64_varint(0);
    writer.write_bool(false);
    writer.write_bool(false);

    writer.write_u64_varint(0);
    writer.write_u32(0);

    writer.write_bool(false);
    writer.write_bool(false);
    writer.write_i8(0);
    writer.write_i32(4);
    writer.write_bool(false);
    writer.write_bool(false);
    writer.write_bool(false);
    writer.write_bool(false);
    writer.write_bool(false);
    writer.write_bool(false);
    writer.write_bool(false);
    writer.write_bool(false);
    writer.write_bool(false);
    writer.write_bool(false);
    writer.write_string(String::from("1.20.72"));
    writer.write_i32(100);
    writer.write_i32(100);
    writer.write_bool(false);
    writer.write_string(String::from(""));
    writer.write_string(String::from(""));
    writer.write_bool(false);
    writer.write_u8(0);
    writer.write_bool(false);

    writer.write_string(String::from(""));
    writer.write_string(String::from("MY SPECIAL LEVEL"));
    writer.write_string(String::from(""));
    writer.write_bool(false);

    // Movement auth not level settings anymore
    writer.write_u8(0);
    writer.write_i64_varint(0);
    writer.write_bool(false);

    writer.write_u64(0);
    writer.write_i32_varint(0);

    writer.write_u32_varint(0);

    writer.write_u32_varint(0);

    writer.write_string(String::from(""));
    writer.write_bool(false);
    writer.write_string(String::from("1.20.72"));

    // Compound tag, I don't know anymore
    writer.write_u8(1);

    writer.write_u64(0);

    // UUID I dunno
    writer.write_u64(0);
    writer.write_u64(0);

    writer.write_bool(false);
    writer.write_bool(false);

    writer.write_bool(false);

    let data = encode(vec![writer.get_payload(GamePacket::StartGame)], conn_info)
        .await
        .unwrap();

    connection_write_half.send(&data, true).await.unwrap();

    // CreativeContent
    let mut writer = PacketWriter::new_game_packet_writer();

    let data = encode(
        vec![writer.get_payload(GamePacket::CreativeContent)],
        conn_info,
    )
    .await
    .unwrap();

    writer.write_u32_varint(0);

    //connection_write_half.send(&data, true).await.unwrap();

    // BiomeDefinitionList
    let mut writer = PacketWriter::new_game_packet_writer();

    let data = encode(
        vec![writer.get_payload(GamePacket::BiomeDefinitionList)],
        conn_info,
    )
    .await
    .unwrap();

    writer.write_u8(0);

    //connection_write_half.send(&data, true).await.unwrap();

    // LevelChunk
    let mut writer = PacketWriter::new_game_packet_writer();

    let data = encode(vec![writer.get_payload(GamePacket::LevelChunk)], conn_info)
        .await
        .unwrap();

    // ChunkPos
    writer.write_i32_varint(0);
    writer.write_i32_varint(0);
    writer.write_i32_varint(0);

    writer.write_i32_varint(0);

    //connection_write_half.send(&data, true).await.unwrap();

    Ok(())
}
