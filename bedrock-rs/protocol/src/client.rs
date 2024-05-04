use crate::gamepacket::GamePacket;
use rak_rs::connection::Connection;

pub struct Client {
    rak_connection: Connection,
}

impl Client {
    pub fn handle_login() {}

    pub fn send_gamepackets(game_packets: Vec<GamePacket>) {}
}
