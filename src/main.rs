use crate::checker::check_startup_files::check_all;
use crate::network::packet_info::GamePacket;
use crate::network::packet_io::PacketWriter;
use crate::server::Server;
use rust_raknet::Reliability;
use std::net::{IpAddr, SocketAddr};
use std::process::exit;
use std::str::FromStr;

mod checker;
mod config;
mod conn;
mod error;
mod logger;
mod network;
mod player;
mod server;
mod utils;

#[tokio::main]
async fn main() {
    //let socket = rust_raknet::RaknetSocket::connect(&SocketAddr::new(
    //    IpAddr::from_str("127.0.0.1").expect("START"),
    //    19132,
    //))
    //.await
    //.expect("TODO: panic message");

    //socket
    //    .send(&*vec![254, 6, 193, 1, 0, 0, 2, 118], Reliability::Reliable)
    //    .await
    //    .expect("SEND");

    //let data = socket.recv().await.expect("TODO: panic message");

    //println!("{:?}", data);

    //let mut writer = PacketWriter::new_game_packet_writer(GamePacket::NETWORK_SETTINGS);

    //writer.write_u16(0);
    //writer.write_u16(0);
    //writer.write_bool(false);
    //writer.write_u8(0);
    //writer.write_f32(0.0);

    //println!("{:?}", writer.get_payload());

    check_all().await;

    Server::start().await;
    Server::stop().await;

    exit(0) // We need to use exit bcz the stdin func from the cli blocks the normal exit
}
