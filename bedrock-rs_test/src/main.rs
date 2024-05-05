use std::net::SocketAddrV4;
use std::str::FromStr;

use bedrock_rs::core::types::u16le;
use bedrock_rs::protocol::gamepacket::GamePacket::NetworkSettings;
use bedrock_rs::protocol::listener::ListenerConfig;
use bedrock_rs::protocol::packets::network_settings::NetworkSettingsPacket;
use tokio::main;
use varint_rs;

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
        SocketAddrV4::from_str("127.0.0.1:19132").unwrap(),
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

