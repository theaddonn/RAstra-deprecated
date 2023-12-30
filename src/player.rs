use crate::utils::lang::Lang;
use uuid::Uuid;
use crate::conn::Conn;

struct Player {
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
    connection: Option<Conn>,
}
