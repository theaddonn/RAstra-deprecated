use uuid::Uuid;

use crate::error::RastraError;
use crate::network::conn_info::ConnInfo;
use crate::network::connection::connection::ConnectionWriteHalf;
use crate::utils::device::Device;
use crate::utils::lang::Lang;
use crate::utils::skin::skin::Skin;

pub struct Player {
    pub name: &'static str,
    pub xuid: &'static str,
    pub uuid: Uuid,
    pub title_id: &'static str,
    pub game_version: &'static str,
    pub current_input_mode: u32,
    pub default_input_mode: u32,
    pub compatible_with_client_side_chunk_gen: bool,
    pub gui_scale: i32,
    pub is_editor_mode: bool,
    pub language: Lang,
    pub skin: Option<Skin>,
    pub device: Option<Device>,
    pub connection_write_half: Option<ConnectionWriteHalf>,
    pub connection_info: Option<ConnInfo>,
}

impl Player {
    pub fn send_text() {}

    pub async fn kick(
        &mut self,
        _message: &str,
        _hide_disconnect_screen: bool,
    ) -> Result<(), RastraError> {
        if self.connection_write_half.is_some() {
            self.connection_write_half
                .as_ref()
                .unwrap()
                .send(&[], false)
                .await
                .unwrap();
            Ok(())
        } else {
            Err(RastraError::PlayerNotOnlineError)
        }
    }
}
