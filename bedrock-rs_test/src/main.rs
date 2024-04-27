use std::io::Cursor;

use bedrock_rs::core::de::MCDeserialize;
use bedrock_rs::core::ser::MCSerialize;
use bedrock_rs::core::types::u16le;
use bedrock_rs::protocol::info::GamePacketType;
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
        name: String::from("Bedrock Test"),
        sub_name: String::from("Rastra"),
        protocol: bedrock_rs::protocol::info::PROTOCOL_VERSION as u16,
        version: String::from("1.20.80"),
        player_count: 1,
        player_max: 10,
        gamemode: Gamemode::Survival,
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

    let pk = NetworkSettingsRequestPacket::deserialize(
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

    nspk_buf.write_u64_varint(GamePacketType::NetworkSettings as u64).unwrap();
    nspk_buf.put_slice(&*nspk.serialize().unwrap());

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

    let pk = LoginPacket::deserialize(
        &mut data
    ).unwrap();

    println!("{:#?}", pk);

    loop {}
}

