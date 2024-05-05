use std::ffi::c_ushort;
use std::io::{Cursor, Read};
use std::net::{Ipv4Addr, SocketAddrV4};
use std::ops::Index;
use std::str::FromStr;

use bedrock_rs::protocol::de::MCProtoDeserialize;
use bedrock_rs::protocol::ser::MCProtoSerialize;
use bedrock_rs::core::types::u16le;
use bedrock_rs::protocol::gamepacket::GamePacket;
use bedrock_rs::protocol::gamepacket::GamePacket::{Login, NetworkSettings};
use bedrock_rs::protocol::listener::ListenerConfig;
use bedrock_rs::protocol::packets::login::LoginPacket;
use bedrock_rs::protocol::packets::network_settings::NetworkSettingsPacket;
use bedrock_rs::protocol::packets::network_settings_request::NetworkSettingsRequestPacket;
use bytes::{Buf, BufMut};
use rak_rs::mcpe::motd::Gamemode;
use rak_rs::Motd;
use rak_rs::server::PossiblySocketAddr::Str;
use tokio::main;
use varint_rs;
use varint_rs::{VarintReader, VarintWriter};

#[main]
async fn main() {

    let mut listener = bedrock_rs::protocol::listener::Listener::new(
        ListenerConfig {
            name: String::from("My Server"),
            sub_name: String::from("bedrock-rs"),
            player_count_max: 10,
            player_count_current: 0,
            nintendo_limited: false,
        },
        SocketAddrV4::from_str("127.0.0.1:19132").unwrap()
    ).await.unwrap();

    listener.start().await.unwrap();

    let mut conn = listener.accept().await.unwrap();

    println!("started!");

    let pk = conn.recv_gamepackets().await.unwrap();

    println!("PKS: {:?}", pk);

    conn.send_gamepackets(vec![NetworkSettings(NetworkSettingsPacket {
        compression_threshold: u16le(0),
        compression_algorythm: u16le(0xFFFF),
        client_throttle_enabled: false,
        client_throttle_threshold: 0,
        client_throttle_scalar: 0.0,
    })]).await.unwrap();

    let pk = conn.recv_gamepackets().await.unwrap();

    println!("PKS: {:?}", pk);

    loop {}
}

