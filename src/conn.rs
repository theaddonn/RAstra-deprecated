use std::io::Cursor;
use crate::raknet::RaknetSocket;

pub struct Conn {
    socket: RaknetSocket,
    compressed: bool,
    encrypted: bool,
    salt: [u8; 16],
    key: String
}

impl Conn {
    pub fn new(raknet_socket: RaknetSocket) -> Conn {
        Self {
            socket: raknet_socket,
            compressed: false,
            encrypted: false,
            salt: [0; 16],
            key: String::new()
        }
    }

    pub fn start(&self) {

    }
}