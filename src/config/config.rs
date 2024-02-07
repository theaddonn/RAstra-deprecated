use crate::utils::compression::CompressionMethod;
use serde::Deserialize;
use std::net::IpAddr;
use std::path::Path;

// This is the config struct which contains all the information needed to start & run the server
// Serde toml generates this struct through the rastra.toml in the servers directory
#[derive(Deserialize)]
pub struct Config {
    pub ip: IpAddr,
    pub port_v4: u16,
    pub port_v6: u16, // TODO

    pub name: String,
    pub motd: String,

    pub compression: CompressionMethod,

    pub auto_update_current_players: bool, // TODO
    pub max_players: u32,

    pub debug_messages: bool, // TODO

    pub protocol_version: u16,
    pub force_protocol_version: bool, // TODO
    pub protocol_edition: String,

    pub game_version: String,
    pub force_version: bool, // TODO
}

impl Config {
    pub async fn load() -> Config {
        let data = tokio::fs::read(Path::new("rastra.toml"))
            .await
            .unwrap();

        let config: Self = toml::from_str(&*String::from_utf8_lossy(&*data)).unwrap();

        return config;
    }
}
