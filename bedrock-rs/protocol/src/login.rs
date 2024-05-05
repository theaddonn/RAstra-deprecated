use bedrock_core::types::u16le;
use crate::compression::CompressionMethod;
use crate::conn::Connection;
use crate::error::{ConnectionError, LoginError};
use crate::gamepacket::GamePacket;
use crate::info::PROTOCOL_VERSION;
use crate::packets::network_settings::NetworkSettingsPacket;

pub struct LoginServerSideOptions {
    pub Compression: Box<dyn CompressionMethod>,
    pub Encryption: bool,
    // TODO: impl Microsoft Auth
    pub AuthenticationEnabled: bool,
    /// Even if the client has another protocol version
    pub AllowOtherProtocols: bool,
}

pub async fn handle_login_server_side(connection: &mut Connection, options: LoginServerSideOptions) -> Result<(), LoginError>{
    // Recieve NetworkRequestSettings
    let gamepackets = match connection.recv_gamepackets().await {
        Ok(v) => { v }
        Err(_) => { return Err(LoginError::PacketMissmatch) }
    };

    // If too many or no packets were send the error
    if gamepackets.len() > 1 || gamepackets.len() < 1 {
        return Err(LoginError::PacketMissmatch)
    }

    // Get the clients protocol version
    let client_proto_ver = match gamepackets[0] {
        GamePacket::RequestNetworkSettings(pk) => {
            pk.client_network_version.0
        }
        _ => {
            return Err(LoginError::PacketMissmatch)
        }
    };

    // Look if Protocol versions match and if other protocol versions are allowed
    if !options.AllowOtherProtocols {
        if client_proto_ver != PROTOCOL_VERSION {
            return Err(LoginError::WrongProtocolVersion {
                server: PROTOCOL_VERSION,
                client: client_proto_ver,
            })
        }
    };

    // Send Network Settings
    match connection.send_gamepackets(vec![GamePacket::NetworkSettings(
        NetworkSettingsPacket {
            compression_threshold: u16le(options.Compression.get_threshold()),
            compression_algorythm: u16le(options.Compression.get_IDu16()),
            // TODO: Figure out what all of this is
            client_throttle_enabled: false,
            client_throttle_threshold: 0,
            client_throttle_scalar: 0.0,
        }
    )]).await {
        Ok(_) => {}
        Err(e) => { return Err(LoginError::ConnError(e)) }
    };

    connection.set_compression_method(Some(
       options.Compression
    ));

    // Recieve Login
    let gamepackets = match connection.recv_gamepackets().await {
        Ok(v) => { v }
        Err(_) => { return Err(LoginError::PacketMissmatch) }
    };

    // If too many or no packets were send the error
    if gamepackets.len() > 1 || gamepackets.len() < 1 {
        return Err(LoginError::PacketMissmatch)
    }

    // Get the clients protocol version
    let login_pk = match &gamepackets[0] {
        GamePacket::Login(pk) => {
            pk
        }
        _ => {
            return Err(LoginError::PacketMissmatch)
        }
    };

    Ok(())

}

pub fn handle_login_client_side(connection: &mut Connection) {
    todo!()
}
