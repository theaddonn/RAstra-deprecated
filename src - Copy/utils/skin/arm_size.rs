use crate::error::Error::SERVER_ArmSizeUnknown;
use crate::error::ServerResult;

pub enum ArmSize {
    SLIM,
    WIDE,
}

impl ArmSize {
    pub fn from_str(val: String) -> ServerResult<ArmSize> {
        match val.to_lowercase().as_str() {
            "wide" => Ok(ArmSize::WIDE),
            "slim" => Ok(ArmSize::SLIM),
            _ => Err(SERVER_ArmSizeUnknown),
        }
    }

    pub fn get_string(&self) -> String {
        match self {
            ArmSize::SLIM => String::from("slim"),
            ArmSize::WIDE => String::from("wide"),
        }
    }
}
