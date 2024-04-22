use bytes::{Buf, BufMut};
use crate::error::ProtocolError;
use crate::gamepacket::{Deserialize, Serialize};
use std::io::Cursor;
use bedrock_core::types::i64be;

// i8

impl Serialize for i8 {
    fn serialize(&self) -> Result<Vec<u8>, ProtocolError> where Self: Sized {
        Ok(self.to_le_bytes().to_vec())
    }
}

impl Deserialize for i8 {
    fn deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, ProtocolError> where Self: Sized {
        if data.remaining() < 1 {
            return Err(ProtocolError::ReadPacketBufError)
        }
        else { 
            return Ok(data.get_i8())
        }
    }
}

// u8
impl Serialize for u8 {
    fn serialize(&self) -> Result<Vec<u8>, ProtocolError> where Self: Sized {
        Ok(self.to_le_bytes().to_vec())
    }
}

impl Deserialize for u8 {
    fn deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, ProtocolError> where Self: Sized {
        if data.remaining() < 1 {
            return Err(ProtocolError::ReadPacketBufError)
        }
        else {
            return Ok(data.get_u8())
        }
    }
}

// i16
impl Serialize for bedrock_core::types::i16le {
    fn serialize(&self) -> Result<Vec<u8>, ProtocolError> where Self: Sized {
        Ok(self.data.to_le_bytes().to_vec())
    }
}

impl Deserialize for bedrock_core::types::i16le {
    fn deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, ProtocolError> where Self: Sized {
        if data.remaining() < 2 {
            return Err(ProtocolError::ReadPacketBufError)
        }
        else {
            return Ok(Self {data: data.get_i16_le()})
        }
    }
}

impl Serialize for bedrock_core::types::i16be {
    fn serialize(&self) -> Result<Vec<u8>, ProtocolError> where Self: Sized {
        Ok(self.data.to_be_bytes().to_vec())
    }
}

impl Deserialize for bedrock_core::types::i16be {
    fn deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, ProtocolError> where Self: Sized {
        if data.remaining() < 2 {
            return Err(ProtocolError::ReadPacketBufError)
        }
        else {
            return Ok(Self {data: data.get_i16()})
        }
    }
}

// u16
impl Serialize for bedrock_core::types::u16le {
    fn serialize(&self) -> Result<Vec<u8>, ProtocolError> where Self: Sized {
        Ok(self.data.to_le_bytes().to_vec())
    }
}

impl Deserialize for bedrock_core::types::u16le {
    fn deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, ProtocolError> where Self: Sized {
        if data.remaining() < 2 {
            return Err(ProtocolError::ReadPacketBufError)
        }
        else {
            return Ok(Self {data: data.get_u16_le()})
        }
    }
}

impl Serialize for bedrock_core::types::u16be {
    fn serialize(&self) -> Result<Vec<u8>, ProtocolError> where Self: Sized {
        Ok(self.data.to_be_bytes().to_vec())
    }
}

impl Deserialize for bedrock_core::types::u16be {
    fn deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, ProtocolError> where Self: Sized {
        if data.remaining() < 2 {
            return Err(ProtocolError::ReadPacketBufError)
        }
        else {
            return Ok(Self {data: data.get_u16()})
        }
    }
}

// i32

impl Serialize for bedrock_core::types::i32le {
    fn serialize(&self) -> Result<Vec<u8>, ProtocolError> where Self: Sized {
        Ok(self.data.to_le_bytes().to_vec())
    }
}

impl Deserialize for bedrock_core::types::i32le {
    fn deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, ProtocolError> where Self: Sized {
        if data.remaining() < 4 {
            return Err(ProtocolError::ReadPacketBufError)
        }
        else {
            return Ok(Self {data: data.get_i32_le()})
        }
    }
}

impl Serialize for bedrock_core::types::i32be {
    fn serialize(&self) -> Result<Vec<u8>, ProtocolError> where Self: Sized {
        Ok(self.data.to_be_bytes().to_vec())
    }
}

impl Deserialize for bedrock_core::types::i32be {
    fn deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, ProtocolError> where Self: Sized {
        if data.remaining() < 4 {
            return Err(ProtocolError::ReadPacketBufError)
        }
        else {
            return Ok(Self {data: data.get_i32()})
        }
    }
}

// u32

impl Serialize for bedrock_core::types::u32le {
    fn serialize(&self) -> Result<Vec<u8>, ProtocolError> where Self: Sized {
        Ok(self.data.to_le_bytes().to_vec())
    }
}

impl Deserialize for bedrock_core::types::u32le {
    fn deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, ProtocolError> where Self: Sized {
        if data.remaining() < 4 {
            return Err(ProtocolError::ReadPacketBufError)
        }
        else {
            return Ok(Self {data: data.get_u32_le()})
        }
    }
}

impl Serialize for bedrock_core::types::u32be {
    fn serialize(&self) -> Result<Vec<u8>, ProtocolError> where Self: Sized {
        Ok(self.data.to_be_bytes().to_vec())
    }
}

impl Deserialize for bedrock_core::types::u32be {
    fn deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, ProtocolError> where Self: Sized {
        if data.remaining() < 4 {
            return Err(ProtocolError::ReadPacketBufError)
        }
        else {
            return Ok(Self {data: data.get_u32()})
        }
    }
}

// i64

impl Serialize for bedrock_core::types::i64le {
    fn serialize(&self) -> Result<Vec<u8>, ProtocolError> where Self: Sized {
        Ok(self.data.to_le_bytes().to_vec())
    }
}

impl Deserialize for bedrock_core::types::i64le {
    fn deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, ProtocolError> where Self: Sized {
        if data.remaining() < 8 {
            return Err(ProtocolError::ReadPacketBufError)
        }
        else {
            return Ok(Self {data: data.get_i64_le()})
        }
    }
}

impl Serialize for bedrock_core::types::i64be {
    fn serialize(&self) -> Result<Vec<u8>, ProtocolError> where Self: Sized {
        Ok(self.data.to_be_bytes().to_vec())
    }
}

impl Deserialize for bedrock_core::types::i64be {
    fn deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, ProtocolError> where Self: Sized {
        if data.remaining() < 8 {
            return Err(ProtocolError::ReadPacketBufError)
        }
        else {
            return Ok(Self {data: data.get_i64()})
        }
    }
}

// u64

impl Serialize for bedrock_core::types::u64le {
    fn serialize(&self) -> Result<Vec<u8>, ProtocolError> where Self: Sized {
        Ok(self.data.to_le_bytes().to_vec())
    }
}

impl Deserialize for bedrock_core::types::u64le {
    fn deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, ProtocolError> where Self: Sized {
        if data.remaining() < 8 {
            return Err(ProtocolError::ReadPacketBufError)
        }
        else {
            return Ok(Self {data: data.get_u64_le()})
        }
    }
}

// i128

impl Serialize for bedrock_core::types::i128le {
    fn serialize(&self) -> Result<Vec<u8>, ProtocolError> where Self: Sized {
        Ok(self.data.to_le_bytes().to_vec())
    }
}

impl Deserialize for bedrock_core::types::i128le {
    fn deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, ProtocolError> where Self: Sized {
        if data.remaining() < 16 {
            return Err(ProtocolError::ReadPacketBufError)
        }
        else {
            return Ok(Self {data: data.get_i128_le()})
        }
    }
}

impl Serialize for bedrock_core::types::i128be {
    fn serialize(&self) -> Result<Vec<u8>, ProtocolError> where Self: Sized {
        Ok(self.data.to_be_bytes().to_vec())
    }
}

impl Deserialize for bedrock_core::types::i128be {
    fn deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, ProtocolError> where Self: Sized {
        if data.remaining() < 16 {
            return Err(ProtocolError::ReadPacketBufError)
        }
        else {
            return Ok(Self {data: data.get_i128()})
        }
    }
}

// u128

impl Serialize for bedrock_core::types::u128le {
    fn serialize(&self) -> Result<Vec<u8>, ProtocolError> where Self: Sized {
        Ok(self.data.to_le_bytes().to_vec())
    }
}

impl Deserialize for bedrock_core::types::u128le {
    fn deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, ProtocolError> where Self: Sized {
        if data.remaining() < 16 {
            return Err(ProtocolError::ReadPacketBufError)
        }
        else {
            return Ok(Self {data: data.get_u128_le()})
        }
    }
}

impl Serialize for bedrock_core::types::u128be {
    fn serialize(&self) -> Result<Vec<u8>, ProtocolError> where Self: Sized {
        Ok(self.data.to_be_bytes().to_vec())
    }
}

impl Deserialize for bedrock_core::types::u128be {
    fn deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, ProtocolError> where Self: Sized {
        if data.remaining() < 16 {
            return Err(ProtocolError::ReadPacketBufError)
        }
        else {
            return Ok(Self {data: data.get_u128()})
        }
    }
}

// f32

impl Serialize for bedrock_core::types::f32le {
    fn serialize(&self) -> Result<Vec<u8>, ProtocolError> where Self: Sized {
        Ok(self.data.to_le_bytes().to_vec())
    }
}

impl Deserialize for bedrock_core::types::f32le {
    fn deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, ProtocolError> where Self: Sized {
        if data.remaining() < 4 {
            return Err(ProtocolError::ReadPacketBufError)
        }
        else {
            return Ok(Self {data: data.get_f32_le()})
        }
    }
}

impl Serialize for bedrock_core::types::f32be {
    fn serialize(&self) -> Result<Vec<u8>, ProtocolError> where Self: Sized {
        Ok(self.data.to_be_bytes().to_vec())
    }
}

impl Deserialize for bedrock_core::types::f32be {
    fn deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, ProtocolError> where Self: Sized {
        if data.remaining() < 4 {
            return Err(ProtocolError::ReadPacketBufError)
        }
        else {
            return Ok(Self {data: data.get_f32()})
        }
    }
}

// f64

impl Serialize for bedrock_core::types::f64le {
    fn serialize(&self) -> Result<Vec<u8>, ProtocolError> where Self: Sized {
        Ok(self.data.to_le_bytes().to_vec())
    }
}

impl Deserialize for bedrock_core::types::f64le {
    fn deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, ProtocolError> where Self: Sized {
        if data.remaining() < 8 {
            return Err(ProtocolError::ReadPacketBufError)
        }
        else {
            return Ok(Self {data: data.get_f64_le()})
        }
    }
}

impl Serialize for bedrock_core::types::f64be {
    fn serialize(&self) -> Result<Vec<u8>, ProtocolError> where Self: Sized {
        Ok(self.data.to_be_bytes().to_vec())
    }
}

impl Deserialize for bedrock_core::types::f64be {
    fn deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, ProtocolError> where Self: Sized {
        if data.remaining() < 8 {
            return Err(ProtocolError::ReadPacketBufError)
        }
        else {
            return Ok(Self {data: data.get_f64()})
        }
    }
}

#[test]
fn f64be_test() {
    let data: i64be = 42;

    let bin = data.serialize().unwrap();

    let data2: i64be =  i64be::deserialize(Cursor::new(bin));

    assert_eq!(data2, 42)
}

// uvarint 32

// ivarint 32

// uvarint 64

// ivarint 32

// bool
