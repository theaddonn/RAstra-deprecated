use crate::raknet::RaknetSocket;

pub struct Conn {
    salt: [u8; 16],
    socket: RaknetSocket,
}