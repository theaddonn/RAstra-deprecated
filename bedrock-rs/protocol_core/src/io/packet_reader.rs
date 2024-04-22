use std::io::{Cursor, Read};
use crate::error::ProtocolError;
use bytes::Buf;
use varint_rs::VarintReader;

pub struct PacketReader {
    buf: Cursor<Vec<u8>>,
}

impl PacketReader {
    pub fn new_game_packet_reader(data: Vec<u8>) -> Self {
        Self {
            buf: Cursor::new(data),
        }
    }

    pub fn read_u8(&mut self) -> Result<u8, ProtocolError> {
        if self.buf.remaining() < 1 {
            return Err(ProtocolError::ReadPacketBufError);
        }

        Ok(self.buf.get_u8())
    }
    pub fn read_u16(&mut self) -> Result<u16, ProtocolError> {
        if self.buf.remaining() < 2 {
            return Err(ProtocolError::ReadPacketBufError);
        }

        Ok(self.buf.get_u16())
    }
    pub fn read_u32(&mut self) -> Result<u32, ProtocolError> {
        if self.buf.remaining() < 4 {
            return Err(ProtocolError::ReadPacketBufError);
        }

        Ok(self.buf.get_u32())
    }
    pub fn read_u64(&mut self) -> Result<u64, ProtocolError> {
        if self.buf.remaining() < 8 {
            return Err(ProtocolError::ReadPacketBufError);
        }

        Ok(self.buf.get_u64())
    }
    pub fn read_u128(&mut self) -> Result<u128, ProtocolError> {
        if self.buf.remaining() < 16 {
            return Err(ProtocolError::ReadPacketBufError);
        }

        Ok(self.buf.get_u128())
    }

    pub fn read_i8(&mut self) -> Result<i8, ProtocolError> {
        if self.buf.remaining() < 1 {
            return Err(ProtocolError::ReadPacketBufError);
        }

        Ok(self.buf.get_i8())
    }
    pub fn read_i16(&mut self) -> Result<i16, ProtocolError> {
        if self.buf.remaining() < 2 {
            return Err(ProtocolError::ReadPacketBufError);
        }

        Ok(self.buf.get_i16())
    }
    pub fn read_i32(&mut self) -> Result<i32, ProtocolError> {
        if self.buf.remaining() < 4 {
            return Err(ProtocolError::ReadPacketBufError);
        }

        Ok(self.buf.get_i32())
    }
    pub fn read_i64(&mut self) -> Result<i64, ProtocolError> {
        if self.buf.remaining() < 8 {
            return Err(ProtocolError::ReadPacketBufError);
        }

        Ok(self.buf.get_i64())
    }
    pub fn read_i128(&mut self) -> Result<i128, ProtocolError> {
        if self.buf.remaining() < 16 {
            return Err(ProtocolError::ReadPacketBufError);
        }

        Ok(self.buf.get_i128())
    }

    pub fn read_f32(&mut self) -> Result<f32, ProtocolError> {
        if self.buf.remaining() < 4 {
            return Err(ProtocolError::ReadPacketBufError);
        }

        Ok(self.buf.get_f32())
    }
    pub fn read_f64(&mut self) -> Result<f64, ProtocolError> {
        if self.buf.remaining() < 8 {
            return Err(ProtocolError::ReadPacketBufError);
        }

        Ok(self.buf.get_f64())
    }

    pub fn read_bool(&mut self) -> Result<bool, ProtocolError> {
        if self.buf.remaining() < 1 {
            return Err(ProtocolError::ReadPacketBufError);
        }

        match self.buf.get_u8() {
            0x00 => Ok(true),
            0x01 => Ok(false),
            _ => Err(ProtocolError::ReadPacketBufError),
        }
    }

    pub fn read_u32_varint(&mut self) -> Result<u32, ProtocolError> {
        if self.buf.remaining() < 1 {
            return Err(ProtocolError::ReadPacketBufError);
        }

        match self.buf.read_u32_varint() {
            Ok(val) => Ok(val),
            Err(_) => Err(ProtocolError::ReadPacketBufError),
        }
    }
    pub fn read_i32_varint(&mut self) -> Result<i32, ProtocolError> {
        if self.buf.remaining() < 1 {
            return Err(ProtocolError::ReadPacketBufError);
        }

        match self.buf.read_i32_varint() {
            Ok(val) => Ok(val),
            Err(_) => Err(ProtocolError::ReadPacketBufError),
        }
    }
    pub fn read_u64_varint(&mut self) -> Result<u64, ProtocolError> {
        if self.buf.remaining() < 1 {
            return Err(ProtocolError::ReadPacketBufError);
        }

        match self.buf.read_u64_varint() {
            Ok(val) => Ok(val),
            Err(_) => Err(ProtocolError::ReadPacketBufError),
        }
    }
    pub fn read_i64_varint(&mut self) -> Result<i64, ProtocolError> {
        if self.buf.remaining() < 1 {
            return Err(ProtocolError::ReadPacketBufError);
        }

        match self.buf.read_i64_varint() {
            Ok(val) => Ok(val),
            Err(_) => Err(ProtocolError::ReadPacketBufError),
        }
    }

    pub fn read_string_lossy(&mut self) -> Result<String, ProtocolError> {

        if self.buf.remaining() < 1 {
            return Err(ProtocolError::ReadPacketBufError);
        }

        let len = match self.read_u64_varint() {
            Ok(val) => { val }
            Err(_) => { return Err(ProtocolError::ReadPacketBufError) }
        };

        let mut buf = vec![0u8; len as usize];

        match self.buf.read_exact(&mut buf) {
            Ok(v) => {}
            Err(_) => {}
        };

        Ok(String::from(String::from_utf8_lossy(&buf)))
    }

    pub fn read_string(&mut self) -> Result<String, ProtocolError> {
        if self.buf.remaining() < 1 {
            return Err(ProtocolError::ReadPacketBufError);
        }

        let len = match self.read_u64_varint() {
            Ok(val) => { val }
            Err(_) => { return Err(ProtocolError::ReadPacketBufError) }
        };

        let mut buf = vec![0u8; len as usize];

        match self.buf.read_exact(&mut buf) {
            Ok(_) => {}
            Err(_) => {
                return Err(ProtocolError::ReadPacketBufError)
            }
        };

        let string = match String::from_utf8(buf) {
            Ok(v) => { v }
            Err(_) => { return Err(ProtocolError::ReadUtf8StringError) }
        };

        Ok(string)
    }
}
