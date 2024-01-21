use crate::conn::Conn;
use crate::error::RastraError;
use crate::utils::device::Device;
use crate::utils::lang::Lang;
use crate::utils::skin::skin::Skin;
use rust_raknet::Reliability;
use std::sync::Arc;
use uuid::Uuid;

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
    connection: Option<Arc<Conn>>,
}

impl Player {
    pub fn send_text() {}

    pub fn is_connected(&self) -> bool {
        return self.connection.is_some();
    }

    pub async fn kick(
        &self,
        message: String,
        hide_disconnect_screen: bool,
    ) -> Result<(), RastraError> {
        return if self.is_connected() {
            unimplemented!("THIS IS UNIMPLEMENTED!");

            self.connection
                .unwrap()
                .socket
                .send(&[], Reliability::Reliable)
                .await
                .unwrap();
            Ok(())
        } else {
            Err(RastraError::ServerPlayerNotConnected)
        };
    }
}
