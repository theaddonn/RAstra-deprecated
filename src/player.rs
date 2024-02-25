use rak_rs::connection::Connection;
use uuid::Uuid;

use crate::error::RastraError;
use crate::network::conn_info::ConnInfo;
use crate::utils::device::Device;
use crate::utils::lang::Lang;
use crate::utils::skin::skin::Skin;

pub struct Player {
    name: String,
    xuid: String,
    uuid: Uuid,
    title_id: String,
    game_version: String,
    current_input_mode: u32,
    default_input_mode: u32,
    compatible_with_client_side_chunk_gen: bool,
    gui_scale: i32,
    is_editor_mode: bool,
    language: Lang,
    skin: Skin,
    device: Device,
    connection: Connection,
    connection_info: ConnInfo,
}

impl Player {
    pub fn send_text() {}

    pub async fn kick(
        &self,
        _message: String,
        _hide_disconnect_screen: bool,
    ) -> Result<(), RastraError> {
        self.connection.send(&[], false).await.unwrap();
        Ok(())
    }
}
