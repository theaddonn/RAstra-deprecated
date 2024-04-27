use crate::error::RastraError;
use crate::network::conn_info::ConnInfo;
use crate::network::connection::connection::{ConnectionReadHalf, ConnectionWriteHalf};
use crate::network::login::handshake::handle_handshake;
use crate::network::login::login::handle_login;
use crate::network::login::network_settings::handle_network_settings;
use crate::player::Player;
use crate::server::Server;
use crate::utils::lang::Lang;
use crate::{log_info, log_warning};

macro_rules! error_and_close {
    ($err:expr, $connection_write_half:expr) => {{
        log_info!(format!("Error while trying to conn to player: {:?}", $err));
        $connection_write_half.close().await;
        Err($err)
    }};
}

pub async fn handle_login_process(
    mut connection_read_half: &mut ConnectionReadHalf,
    mut connection_write_half: ConnectionWriteHalf,
) -> Result<Player, RastraError> {
    let mut conn_info = ConnInfo::new();

    let protocol_version = match handle_network_settings(
        &mut connection_read_half,
        &connection_write_half,
        &mut conn_info,
    )
    .await
    {
        Ok(v) => v,
        Err(e) => {
            connection_write_half.close().await;
            return error_and_close!(e, connection_write_half);
        }
    };

    let server = Server::get_instance().await;
    let server = server.lock().await;

    if server.config.force_protocol_version {
        if protocol_version != server.config.protocol_version {
            log_warning!(format!(
                "Client tried to connect with wrong protocol version (Server: {} | Client: {})",
                server.config.protocol_version, protocol_version
            ));
            return error_and_close!(
                RastraError::WrongProtocolVersionError,
                connection_write_half
            );
        }
    }

    if server.config.compression_enabled {
        conn_info.enable_compression(server.config.compression_method);
    }

    drop(server);

    log_info!("OK AFTER NETWORK SETTINGS!");

    let _ = match handle_login(&mut connection_read_half, &conn_info).await {
        Ok(_) => {}
        Err(e) => {
            return error_and_close!(e, connection_write_half);
        }
    };

    log_info!("OK AFTER LOGIN!");

    let _ = match handle_handshake(
        &mut connection_read_half,
        &connection_write_half,
        &mut conn_info,
    )
    .await
    {
        Ok(_) => {
            log_info!("OKI");
        }
        Err(e) => {
            return error_and_close!(e, connection_write_half);
        }
    };

    log_info!("OK AFTER HANDSHAKE!");

    //let _ = match handle_play_status(&mut connection_read_half, &connection_write_half, &mut conn_info, PlayStatus::LoginSuccess).await {
    //    Ok(_) => {}
    //    Err(e) => {
    //        return error_and_close!(e, connection_write_half);
    //    }
    //};
    //
    //log_info!("OK AFTER PLAY STATUS (PlayStatus::LoginSuccess)!");

    //    //let _ = match handle_packs(&connection_write_half, &mut conn_info).await {
    //    Ok(_) => {}
    //    Err(e) => {
    //        return error_and_close!(e, connection_write_half);
    //    }
    //};
    //
    //log_info!("OK AFTER PACKS!");

    //    //let _ = match handle_start_game(&connection_write_half, &mut conn_info).await {
    //    Ok(_) => {}
    //    Err(e) => {
    //        return error_and_close!(e, connection_write_half);
    //    }
    //};

    log_info!("OK AFTER START GAME!");

    let server = Server::get_instance().await;
    let server = server.lock().await;

    let player = Player {
        name: "",
        xuid: "",
        uuid: Default::default(),
        title_id: "",
        game_version: "",
        current_input_mode: 0,
        default_input_mode: 0,
        compatible_with_client_side_chunk_gen: false,
        gui_scale: 0,
        is_editor_mode: false,
        language: Lang::en_US,
        skin: None,
        device: None,
        connection_write_half: None,
        connection_info: None,
    };

    Ok(player)
}
