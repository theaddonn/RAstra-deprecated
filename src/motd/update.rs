use rak_rs::mcpe::motd::Gamemode;
use rak_rs::{Listener, Motd};
use rand::random;

use crate::server::Server;

pub async fn update_motd(listener: &mut Listener) {
    let server = Server::get_instance().await;
    let server = server.lock().await;

    let name = server.config.name.to_owned();
    let protocol = server.config.protocol_version.to_owned();
    let version = server.config.game_version.to_owned();
    //let player_count = server.players.len() as u32;
    let player_max = server.config.max_players.to_owned();
    let player_count = random::<u32>() % player_max;
    let server_guid = server.guid.to_owned();
    let port: String = server.config.port_v4.to_string();
    let ipv6_port: String = server.config.port_v6.to_string();

    listener.motd = Motd {
        name,
        protocol,
        version,
        player_count,
        player_max,
        gamemode: Gamemode::Survival,
        server_guid,
        port,
        ipv6_port,
    }
}
