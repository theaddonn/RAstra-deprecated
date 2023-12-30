use crate::utils::color::Color;
use crate::utils::skin::arm_size::ArmSize;

struct Skin {
    arm_size: ArmSize,
    persona: bool,
    play_fab_id: String,
    premium: bool,
    color: Color,
    data: String,
    geometry_data: String,
    geometry_data_engine_version: String,
    id: String,
    image_height: u32,
    image_width: u32,
    resource_patch: String,
    trusted: bool,
}
