use std::ffi::c_ushort;
use std::io::{Cursor, Read};
use std::net::{Ipv4Addr, SocketAddrV4};
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

    listener.start()


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

    nspk_buf.write_u64_varint(NetworkSettings(nspk).id()).unwrap();
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

