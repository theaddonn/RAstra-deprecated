extern crate core;

use std::process::exit;

use crate::checker::check_startup_files::check_all;
use crate::network::conn_info::ConnInfo;
use crate::network::utils::handler::decode;
use crate::server::Server;
use crate::utils::compression_method::CompressionMethod;

mod checker;
mod cli;
mod config;
mod error;
mod gamemode;
mod logger;
mod motd;
mod network;
mod player;
mod server;
mod tick;
mod utils;

#[tokio::main]
async fn main() {
    let compressed_server_to_client_handshake_packet: Vec<u8> = vec![
        254, 0, 1, 188, 1, 67, 254, 186, 3, 3, 183, 3, 101, 121, 74, 48, 101, 88, 65, 105, 79, 105,
        74, 75, 86, 49, 81, 105, 76, 67, 74, 104, 98, 71, 99, 105, 79, 105, 74, 70, 85, 122, 77,
        52, 78, 67, 73, 115, 73, 110, 103, 49, 100, 83, 73, 54, 73, 107, 49, 73, 87, 88, 100, 70,
        81, 86, 108, 73, 83, 50, 57, 97, 83, 88, 112, 113, 77, 69, 78, 66, 85, 86, 108, 71, 83,
        122, 82, 70, 82, 85, 70, 68, 83, 85, 82, 90, 90, 48, 70, 70, 100, 88, 104, 108, 87, 107,
        56, 51, 83, 69, 49, 108, 89, 86, 100, 117, 83, 109, 108, 115, 78, 50, 103, 114, 84, 72,
        104, 111, 89, 107, 74, 97, 98, 107, 108, 82, 82, 51, 103, 119, 86, 107, 85, 51, 98, 106,
        90, 53, 75, 50, 104, 48, 98, 50, 82, 110, 82, 48, 49, 115, 98, 86, 74, 78, 77, 87, 57, 77,
        83, 86, 73, 119, 99, 86, 86, 71, 98, 85, 115, 51, 83, 122, 90, 74, 77, 85, 107, 49, 75,
        121, 116, 69, 89, 107, 81, 53, 81, 48, 112, 52, 97, 110, 66, 72, 77, 122, 69, 119, 98, 69,
        49, 104, 99, 70, 103, 121, 87, 109, 82, 76, 82, 48, 120, 72, 97, 122, 78, 75, 76, 48, 85,
        121, 84, 72, 86, 67, 99, 122, 70, 86, 100, 86, 100, 97, 87, 87, 90, 80, 90, 88, 103, 121,
        77, 49, 77, 53, 78, 69, 82, 50, 77, 106, 86, 85, 100, 70, 86, 48, 82, 51, 99, 105, 102, 81,
        46, 101, 121, 74, 122, 89, 87, 120, 48, 73, 106, 111, 105, 78, 50, 86, 109, 84, 51, 107,
        53, 101, 87, 85, 118, 97, 85, 53, 77, 101, 69, 120, 113, 97, 106, 74, 116, 78, 86, 108, 82,
        100, 122, 48, 57, 73, 110, 48, 46, 78, 53, 51, 50, 51, 105, 51, 82, 112, 84, 115, 76, 57,
        95, 100, 85, 75, 118, 50, 70, 122, 118, 83, 69, 71, 105, 111, 116, 97, 76, 89, 54, 85, 48,
        76, 79, 57, 83, 69, 56, 53, 80, 107, 87, 65, 86, 102, 100, 84, 77, 68, 48, 85, 72, 69, 66,
        105, 120, 48, 55, 95, 106, 84, 107, 50, 56, 87, 117, 71, 74, 104, 76, 102, 111, 57, 55, 97,
        87, 112, 99, 69, 113, 104, 84, 101, 71, 99, 101, 113, 108, 97, 95, 74, 67, 86, 73, 106,
        106, 116, 110, 97, 77, 98, 105, 108, 112, 118, 113, 54, 68, 118, 54, 80, 74, 109, 87, 111,
        69, 108, 79, 85, 98, 115, 116, 102, 83, 90, 101,
    ];

    let mut conn_info = ConnInfo::new();

    conn_info.enable_compression(CompressionMethod::Flate);

    let data = decode(compressed_server_to_client_handshake_packet, &conn_info)
        .await
        .unwrap();

    let mut reader = PacketReader::new_game_packet_reader(data[0].clone());

    reader.read_u64_varint().unwrap();

    //println!("{}", reader.read_string_lossy().unwrap());

    check_all().await;

    Server::start().await;
    Server::stop().await;

    exit(0) // We need to use exit bcz the stdin func from the cli blocks the normal exit
}
