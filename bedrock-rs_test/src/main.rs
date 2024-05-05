use std::net::SocketAddrV4;
use std::str::FromStr;

use bedrock_rs::protocol::compression::snappy::SnappyCompression;
use bedrock_rs::protocol::listener::ListenerConfig;
use bedrock_rs::protocol::login::{handle_login_server_side, LoginServerSideOptions};
use tokio::main;

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

    handle_login_server_side(&mut conn, LoginServerSideOptions {
        Compression: Box::new(SnappyCompression{threshold: 1024}),
        Encryption: false,
        AuthenticationEnabled: false,
        AllowOtherProtocols: false,
    }).await.unwrap();

    println!("login successful!");

    loop {}
}

