use std::process::exit;

use crate::checker::check_startup_files::check_all;
use crate::server::Server;

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
mod utils;

#[tokio::main]
async fn main() {
    check_all().await;

    Server::start().await;
    Server::stop().await;

    exit(0) // We need to use exit bcz the stdin func from the cli blocks the normal exit
}
