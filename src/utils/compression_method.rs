use serde::Deserialize;

use crate::error::RastraError;

#[derive(Deserialize, Copy, Clone)]
pub enum CompressionMethod {
    Flate = 0,
    Snappy = 1,
}

impl CompressionMethod {
    pub fn to_int(&self) -> i16 {
        match self {
            CompressionMethod::Flate => 0x00,
            CompressionMethod::Snappy => 0x01,
        }
    }

    pub fn from_int(val: i16) -> Result<CompressionMethod, RastraError> {
        match val {
            0x00 => Ok(CompressionMethod::Flate),
            0x01 => Ok(CompressionMethod::Snappy),
            _ => Err(RastraError::CompressionMethodUnknown),
        }
    }
}
