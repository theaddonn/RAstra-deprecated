use crate::checker::check_startup_files::check_all;
use crate::server::Server;
use std::process::exit;

mod checker;
mod config;
mod error;
mod logger;
mod network;
mod player;
mod server;
mod utils;
mod gamemode;
mod cli;
mod motd;

#[tokio::main]
async fn main() {
    check_all().await;

    Server::start().await;
    Server::stop().await;

    exit(0) // We need to use exit bcz the stdin func from the cli blocks the normal exit
}
