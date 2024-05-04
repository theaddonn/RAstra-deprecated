use rak_rs::connection::Connection;
use crate::gamepacket::GamePacket;

pub struct Client {
    rak_connection: Connection,
}

impl Client {
    pub fn handle_login() {

    }

    pub fn send_gamepackets(game_packets: Vec<GamePacket>) {}
}