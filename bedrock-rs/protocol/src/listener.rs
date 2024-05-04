use std::net::{SocketAddrV4, SocketAddrV6};
use crate::error::ListenerError;

pub struct Listener {
    rak_listener: rak_rs::Listener,
    pub config: ListenerConfig,
}

impl Listener {
    pub async fn build(listener_config: ListenerOptions) -> Result<Self, ListenerError> {
        let rak_listener = rak_rs::Listener::bind(listener_config.socket_addr_v4).await;

        let rak_listener = match rak_listener {
            Ok(v) => { v }
            Err(_) => { return Err(ListenerError::AddrBindErr) }
        };



    }

    pub fn start(&mut self) {
        self.rak_listener.start().await;
    }


    pub fn accept(&mut self) {

    }
}

pub struct ListenerOptions {
    name: String,
    sub_name: String,
    socket_addr_v4: SocketAddrV4,
    socket_addr_v6: SocketAddrV6,
}

