use crate::error::ListenerError;
use core::net::SocketAddr;
use core::net::{SocketAddrV4, SocketAddrV6};

pub struct Listener {
    rak_listener: rak_rs::Listener,
    config: ListenerConfig,
}

impl Listener {
    pub async fn build(listener_config: ListenerConfig) -> Result<Self, ListenerError> {
        let rak_listener =
            rak_rs::Listener::bind(SocketAddr::V4(listener_config.socket_addr_v4.clone())).await;

        let rak_listener = match rak_listener {
            Ok(v) => v,
            Err(_) => return Err(ListenerError::AddrBindErr),
        };

        Ok(Self {
            rak_listener,
            config: listener_config,
        })
    }

    pub async fn start(&mut self) {
        self.rak_listener.start().await;
    }

    pub fn accept(&mut self) {}

    fn get_options(&self) -> &ListenerConfig {
        &self.config
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ListenerConfig {
    pub name: String,
    pub sub_name: String,
    pub socket_addr_v4: SocketAddrV4,
    pub socket_addr_v6: SocketAddrV6,
}
