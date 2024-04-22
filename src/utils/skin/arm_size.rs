use crate::error::RastraError;
use crate::error::RastraError::ArmSizeUnknown;

#[derive(Debug)]
pub enum ArmSize {
    SLIM,
    WIDE,
}

impl ArmSize {
    pub fn from_str(val: String) -> Result<ArmSize, RastraError> {
        match val.to_lowercase().as_str() {
            "wide" => Ok(ArmSize::WIDE),
            "slim" => Ok(ArmSize::SLIM),
            _ => Err(ArmSizeUnknown),
        }
    }

    pub fn get_string(&self) -> String {
        match self {
            ArmSize::SLIM => String::from("slim"),
            ArmSize::WIDE => String::from("wide"),
        }
    }
}
