use rak_rs::connection::Connection;

#[derive(Debug, Copy, Clone)]
pub enum ConnState {
    ConnectionAccepted,
    NetworkSettings,
    Login,
    ActivePlay,
}

pub struct ConnInfo {
    pub socket: Connection,
    pub compressed: bool,
    pub encrypted: bool,
    pub salt: [u8; 16],
    pub key: String,
    pub state: ConnState,
}
