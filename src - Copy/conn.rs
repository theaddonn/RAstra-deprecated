use crate::raknet::{RaknetSocket, Reliability};
use std::sync::Arc;
use crate::conn::ConnState::CONNECTION_ACCEPTED;
use crate::error::ServerResult;
use crate::player::Player;

pub enum ConnState {
    CONNECTION_ACCEPTED,
    NETWORK_SETTINGS,
    LOGIN,
    ACTIVE_PLAY,
}

pub struct Conn {
    pub socket: RaknetSocket,
    compressed: bool,
    encrypted: bool,
    salt: [u8; 16],
    key: String,
    state: ConnState,
    pub player: Option<Arc<Player>>
}

impl Conn {
    pub fn start_new(raknet_socket: RaknetSocket) -> Conn {
        Self {
            socket: raknet_socket,
            compressed: false,
            encrypted: false,
            salt: [0; 16],
            key: String::new(),
            state: CONNECTION_ACCEPTED,
            player: None,
        }
    }

    pub async fn manage(&mut self){
        self.network_settings().await;
        self.login_process().await;
        self.handle_packets().await;
    }

    pub async fn network_settings(&mut self){
        self.state = ConnState::NETWORK_SETTINGS

    }

    pub async fn login_process(&mut self){
        self.state = ConnState::LOGIN

    }

    pub async fn handle_packets(&self){

    }



    pub async fn send(&self, data: &[u8]){

    }
}