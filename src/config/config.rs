use std::net::IpAddr;
use std::path::Path;

use serde::Deserialize;

use crate::log_error;
use crate::utils::compression_method::CompressionMethod;

// This is the config struct which contains all the information needed to start & run the server
// Serde toml generates this struct through the rastra.toml in the servers directory
#[derive(Deserialize)]
pub struct Config {
    pub ip: IpAddr,
    pub port_v4: u16,
    // TODO
    pub port_v6: u16,

    pub name: String,
    pub motd: String,

    pub auto_update_current_players: bool,
    // TODO
    pub max_players: u32,

    // TODO
    pub debug_messages: bool,

    // TODO
    pub compression_enabled: bool,
    pub protocol_version: u32,
    pub force_protocol_version: bool,
    // TODO
    pub protocol_edition: String,

    pub compression_threshold: i16,
    // TODO
    pub compression_method: CompressionMethod,
    // TODO
    pub client_throttle_enabled: bool,
    // TODO
    pub client_throttle_threshold: u8,
    // TODO
    pub client_throttle_scalar: f32,
    // TODO
    pub game_version: String,
}

impl Config {
    pub async fn load() -> Config {
        let data = tokio::fs::read(Path::new("rastra.toml")).await.unwrap();

        let config: Self = match toml::from_str(&*String::from_utf8_lossy(&*data)) {
            Ok(v) => { v },
            Err(e) => {
                log_error!(format!("Couldn't load rastra.toml: {}", e))
            }
        };

        return config;
    }
}
