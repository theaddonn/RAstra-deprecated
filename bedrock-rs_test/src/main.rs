use std::io::{Cursor, Read};

use bedrock_rs::protocol::de::MCProtoDeserialize;
use bedrock_rs::protocol::ser::MCProtoSerialize;
use bedrock_rs::core::types::u16le;
use bedrock_rs::protocol::info::GamePacket;
use bedrock_rs::protocol::packets::login::LoginPacket;
use bedrock_rs::protocol::packets::network_settings::NetworkSettingsPacket;
use bedrock_rs::protocol::packets::network_settings_request::NetworkSettingsRequestPacket;
use bytes::{Buf, BufMut};
use rak_rs::mcpe::motd::Gamemode;
use rak_rs::Motd;
use tokio::main;
use varint_rs;
use varint_rs::{VarintReader, VarintWriter};

#[main]
async fn main() {
    let mut listener = rak_rs::server::Listener::bind("127.0.0.1:19132").await.unwrap();

    listener.motd = Motd {
        edition: String::from("MCPE"),
        name: String::from(format!("Bedrock Test {}", String::from_utf8(vec![0xc2, 0xa7, 0x41, 0xc2, 0xa7, 0x4d, 0xc2, 0xa7, 0x45, 0xc2, 0xa7, 0x54, 0xc2, 0xa7, 0x48, 0xc2, 0xa7, 0x59, 0xc2, 0xa7, 0x53, 0xc2, 0xa7, 0x54]).unwrap())),
        sub_name: String::from("RAstra - v1.20.81 Mommy Edition"),
        protocol: bedrock_rs::protocol::info::PROTOCOL_VERSION as u16,
        version: String::from(""),
        player_count: 1,
        player_max: 10,
        gamemode: Gamemode::Creative,
        server_guid: rand::random(),
        port: Some(String::from("19132")),
        ipv6_port: Some(String::from("19133")),
        nintendo_limited: Some(false),
    };

    listener.versions = &[9, 10, 11];

    listener.id = rand::random();

    listener.start().await.unwrap();

    println!("started!");

    let mut conn = listener.accept().await.unwrap();

    let mut data = Cursor::new(conn.recv().await.unwrap());

    data.get_u8();
    data.read_u64_varint().unwrap();
    data.read_u64_varint().unwrap();

    let pk = NetworkSettingsRequestPacket::proto_deserialize(
        &mut data
    ).unwrap();

    println!("{:#?}", pk);

    let nspk = NetworkSettingsPacket {
        compression_threshold: u16le(0),
        compression_algorythm: u16le(1),
        client_throttle_enabled: false,
        client_throttle_threshold: 0,
        client_throttle_scalar: 0.0,
    };

    let mut nspk_buf = vec![];

    nspk_buf.write_u64_varint(GamePacket::NetworkSettings as u64).unwrap();
    nspk.proto_serialize(&mut nspk_buf).unwrap();

    let mut buf = vec![];
    buf.put_u8(bedrock_rs::protocol::info::RAKNET_GAME_PACKET_ID);
    buf.put_u8(0);
    buf.write_u64_varint(nspk_buf.len() as u64).unwrap();

    buf.put_slice(&*nspk_buf);

    conn.send(&*buf, false).await.unwrap();

    println!("{:#?}", nspk);

    let mut data = Cursor::new(conn.recv().await.unwrap());

    data.get_u8();
    data.get_u8();
    data.read_u64_varint().unwrap();
    data.read_u64_varint().unwrap();

    let pk = LoginPacket::proto_deserialize(
        &mut data
    ).unwrap();

    println!("{:#?}", pk);



    loop {}
}

