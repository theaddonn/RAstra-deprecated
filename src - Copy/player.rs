use std::sync::Arc;
use crate::conn::Conn;
use crate::utils::lang::Lang;
use uuid::Uuid;
use crate::error::{Error, ServerResult};
use crate::network::packet_io::PacketWriter;
use crate::raknet::Reliability;
use crate::utils::device::Device;
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
    connection: Option<Arc<Conn>>,
}

impl Player {

    pub fn send_text(){}

    pub fn is_online(&self) -> bool {
        return self.connection.is_some();
    }

    pub async fn kick(&self, message: String, hide_disconnect_screen: bool) -> ServerResult<()> {
        if self.connection.is_some()  {
            let mut writer = PacketWriter::new();
            writer.write_u8(0x05);
            writer.write_bool(hide_disconnect_screen);
            writer.write_string(&*message);
            match self.connection.as_ref().unwrap().socket.send(&*writer.get_raw_payload(), Reliability::Reliable).await {
                Err(err) => Err(err),
                Ok(_) => Ok(()),
            }
        }
        else {
            Err(Error::SERVER_PlayerOffline)
        }
    }
}
