use crate::server::Server;
use std::env;
use std::io::Read;
use std::process::exit;
use std::str::FromStr;
use varint_rs::VarintReader;
use varint_rs::VarintWriter;
use crate::raknet::log::enable_raknet_log;

mod conn;
mod error;
mod logger;
mod network;
mod player;
mod raknet;
mod server;
mod utils;

#[tokio::main]
async fn main() {
    env::set_var("RUST_BACKTRACE", "full");
    enable_raknet_log(04);

    let mut server = Server::new().await;

    Server::start(&server).await;
    Server::stop(&server).await;
    exit(0) // We need to use exit bcz the stdin func from the cli blocks the normal exit
}
