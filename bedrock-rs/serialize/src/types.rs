use std::io::{Cursor, Read};

use bytes::Buf;
use bytes::BufMut;
use varint_rs::{VarintReader, VarintWriter};

use crate::de::MCDeserialize;
use crate::error::{DeserilizationError, SerilizationError};
use crate::ser::MCSerialize;

// i8
impl MCSerialize for i8 {
    fn serialize(&self) -> Result<Vec<u8>, SerilizationError>
    where
        Self: Sized,
    {
        Ok(self.to_le_bytes().to_vec())
    }
}

impl MCDeserialize for i8 {
    fn deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized,
    {
        if data.remaining() < 1 {
            return Err(DeserilizationError::NotEnoughtRemainingError);
        } else {
            return Ok(data.get_i8());
        }
    }
}

// u8
impl MCSerialize for u8 {
    fn serialize(&self) -> Result<Vec<u8>, SerilizationError>
    where
        Self: Sized,
    {
        Ok(self.to_le_bytes().to_vec())
    }
}

impl MCDeserialize for u8 {
    fn deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized,
    {
        if data.remaining() < 1 {
            return Err(DeserilizationError::NotEnoughtRemainingError);
        } else {
            return Ok(data.get_u8());
        }
    }
}

// i16
impl MCSerialize for bedrock_core::types::i16le {
    fn serialize(&self) -> Result<Vec<u8>, SerilizationError>
    where
        Self: Sized,
    {
        Ok(self.0.to_le_bytes().to_vec())
    }
}

impl MCDeserialize for bedrock_core::types::i16le {
    fn deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized,
    {
        if data.remaining() < 2 {
            return Err(DeserilizationError::NotEnoughtRemainingError);
        } else {
            return Ok(Self(data.get_i16_le()));
        }
    }
}

impl MCSerialize for bedrock_core::types::i16be {
    fn serialize(&self) -> Result<Vec<u8>, SerilizationError>
    where
        Self: Sized,
    {
        Ok(self.0.to_be_bytes().to_vec())
    }
}

impl MCDeserialize for bedrock_core::types::i16be {
    fn deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized,
    {
        if data.remaining() < 2 {
            return Err(DeserilizationError::NotEnoughtRemainingError);
        } else {
            return Ok(Self(data.get_i16()));
        }
    }
}

// u16
impl MCSerialize for bedrock_core::types::u16le {
    fn serialize(&self) -> Result<Vec<u8>, SerilizationError>
    where
        Self: Sized,
    {
        Ok(self.0.to_le_bytes().to_vec())
    }
}

impl MCDeserialize for bedrock_core::types::u16le {
    fn deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized,
    {
        if data.remaining() < 2 {
            return Err(DeserilizationError::NotEnoughtRemainingError);
        } else {
            return Ok(Self(data.get_u16_le()));
        }
    }
}

impl MCSerialize for bedrock_core::types::u16be {
    fn serialize(&self) -> Result<Vec<u8>, SerilizationError>
    where
        Self: Sized,
    {
        Ok(self.0.to_be_bytes().to_vec())
    }
}

impl MCDeserialize for bedrock_core::types::u16be {
    fn deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized,
    {
        if data.remaining() < 2 {
            return Err(DeserilizationError::NotEnoughtRemainingError);
        } else {
            return Ok(Self(data.get_u16()));
        }
    }
}

// i32

impl MCSerialize for bedrock_core::types::i32le {
    fn serialize(&self) -> Result<Vec<u8>, SerilizationError>
    where
        Self: Sized,
    {
        Ok(self.0.to_le_bytes().to_vec())
    }
}

impl MCDeserialize for bedrock_core::types::i32le {
    fn deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized,
    {
        if data.remaining() < 4 {
            return Err(DeserilizationError::NotEnoughtRemainingError);
        } else {
            return Ok(Self(data.get_i32_le()));
        }
    }
}

impl MCSerialize for bedrock_core::types::i32be {
    fn serialize(&self) -> Result<Vec<u8>, SerilizationError>
    where
        Self: Sized,
    {
        Ok(self.0.to_be_bytes().to_vec())
    }
}

impl MCDeserialize for bedrock_core::types::i32be {
    fn deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized,
    {
        if data.remaining() < 4 {
            return Err(DeserilizationError::NotEnoughtRemainingError);
        } else {
            return Ok(Self(data.get_i32()));
        }
    }
}

// u32

impl MCSerialize for bedrock_core::types::u32le {
    fn serialize(&self) -> Result<Vec<u8>, SerilizationError>
    where
        Self: Sized,
    {
        Ok(self.0.to_le_bytes().to_vec())
    }
}

impl MCDeserialize for bedrock_core::types::u32le {
    fn deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized,
    {
        if data.remaining() < 4 {
            return Err(DeserilizationError::NotEnoughtRemainingError);
        } else {
            return Ok(Self(data.get_u32_le()));
        }
    }
}

impl MCSerialize for bedrock_core::types::u32be {
    fn serialize(&self) -> Result<Vec<u8>, SerilizationError>
    where
        Self: Sized,
    {
        Ok(self.0.to_be_bytes().to_vec())
    }
}

impl MCDeserialize for bedrock_core::types::u32be {
    fn deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized,
    {
        if data.remaining() < 4 {
            return Err(DeserilizationError::NotEnoughtRemainingError);
        } else {
            return Ok(Self(data.get_u32()));
        }
    }
}

// i64

impl MCSerialize for bedrock_core::types::i64le {
    fn serialize(&self) -> Result<Vec<u8>, SerilizationError>
    where
        Self: Sized,
    {
        Ok(self.0.to_le_bytes().to_vec())
    }
}

impl MCDeserialize for bedrock_core::types::i64le {
    fn deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized,
    {
        if data.remaining() < 8 {
            return Err(DeserilizationError::NotEnoughtRemainingError);
        } else {
            return Ok(Self(data.get_i64_le()));
        }
    }
}

impl MCSerialize for bedrock_core::types::i64be {
    fn serialize(&self) -> Result<Vec<u8>, SerilizationError>
    where
        Self: Sized,
    {
        Ok(self.0.to_be_bytes().to_vec())
    }
}

impl MCDeserialize for bedrock_core::types::i64be {
    fn deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized,
    {
        if data.remaining() < 8 {
            return Err(DeserilizationError::NotEnoughtRemainingError);
        } else {
            return Ok(Self(data.get_i64()));
        }
    }
}

// u64

impl MCSerialize for bedrock_core::types::u64le {
    fn serialize(&self) -> Result<Vec<u8>, SerilizationError>
    where
        Self: Sized,
    {
        Ok(self.0.to_le_bytes().to_vec())
    }
}

impl MCDeserialize for bedrock_core::types::u64le {
    fn deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized,
    {
        if data.remaining() < 8 {
            return Err(DeserilizationError::NotEnoughtRemainingError);
        } else {
            return Ok(Self(data.get_u64_le()));
        }
    }
}

// i128

impl MCSerialize for bedrock_core::types::i128le {
    fn serialize(&self) -> Result<Vec<u8>, SerilizationError>
    where
        Self: Sized,
    {
        Ok(self.0.to_le_bytes().to_vec())
    }
}

impl MCDeserialize for bedrock_core::types::i128le {
    fn deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized,
    {
        if data.remaining() < 16 {
            return Err(DeserilizationError::NotEnoughtRemainingError);
        } else {
            return Ok(Self(data.get_i128_le()));
        }
    }
}

impl MCSerialize for bedrock_core::types::i128be {
    fn serialize(&self) -> Result<Vec<u8>, SerilizationError>
    where
        Self: Sized,
    {
        Ok(self.0.to_be_bytes().to_vec())
    }
}

impl MCDeserialize for bedrock_core::types::i128be {
    fn deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized,
    {
        if data.remaining() < 16 {
            return Err(DeserilizationError::NotEnoughtRemainingError);
        } else {
            return Ok(Self(data.get_i128()));
        }
    }
}

// u128

impl MCSerialize for bedrock_core::types::u128le {
    fn serialize(&self) -> Result<Vec<u8>, SerilizationError>
    where
        Self: Sized,
    {
        Ok(self.0.to_le_bytes().to_vec())
    }
}

impl MCDeserialize for bedrock_core::types::u128le {
    fn deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized,
    {
        if data.remaining() < 16 {
            return Err(DeserilizationError::NotEnoughtRemainingError);
        } else {
            return Ok(Self(data.get_u128_le()));
        }
    }
}

impl MCSerialize for bedrock_core::types::u128be {
    fn serialize(&self) -> Result<Vec<u8>, SerilizationError>
    where
        Self: Sized,
    {
        Ok(self.0.to_be_bytes().to_vec())
    }
}

impl MCDeserialize for bedrock_core::types::u128be {
    fn deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized,
    {
        if data.remaining() < 16 {
            return Err(DeserilizationError::NotEnoughtRemainingError);
        } else {
            return Ok(Self(data.get_u128()));
        }
    }
}

// f32

impl MCSerialize for f32 {
    fn serialize(&self) -> Result<Vec<u8>, SerilizationError>
    where
        Self: Sized,
    {
        Ok(self.to_le_bytes().to_vec())
    }
}

impl MCDeserialize for f32 {
    fn deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized,
    {
        if data.remaining() < 4 {
            return Err(DeserilizationError::NotEnoughtRemainingError);
        } else {
            return Ok(data.get_f32_le());
        }
    }
}

// f64

impl MCSerialize for f64 {
    fn serialize(&self) -> Result<Vec<u8>, SerilizationError>
    where
        Self: Sized,
    {
        Ok(self.to_le_bytes().to_vec())
    }
}

impl MCDeserialize for f64 {
    fn deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized,
    {
        if data.remaining() < 8 {
            return Err(DeserilizationError::NotEnoughtRemainingError);
        } else {
            return Ok(data.get_f64_le());
        }
    }
}

// uvarint 32

impl MCSerialize for bedrock_core::types::uvar32 {
    fn serialize(&self) -> Result<Vec<u8>, SerilizationError>
    where
        Self: Sized,
    {
        let mut buf = vec![];

        match buf.write_u32_varint(self.0) {
            Ok(v) => v,
            Err(_) => return Err(SerilizationError::WriteVarintError),
        };

        Ok(buf)
    }
}

impl MCDeserialize for bedrock_core::types::uvar32 {
    fn deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized,
    {
        return Ok(Self(match data.read_u32_varint() {
            Ok(v) => v,
            Err(_) => return Err(DeserilizationError::ReadVarintError),
        }));
    }
}

// ivarint 32

impl MCSerialize for bedrock_core::types::ivar32 {
    fn serialize(&self) -> Result<Vec<u8>, SerilizationError>
    where
        Self: Sized,
    {
        let mut buf = vec![];

        match buf.write_i32_varint(self.0) {
            Ok(v) => v,
            Err(_) => return Err(SerilizationError::WriteVarintError),
        };

        Ok(buf)
    }
}

impl MCDeserialize for bedrock_core::types::ivar32 {
    fn deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized,
    {
        return Ok(Self(match data.read_i32_varint() {
            Ok(v) => v,
            Err(_) => return Err(DeserilizationError::ReadVarintError),
        }));
    }
}

// uvarint 64

impl MCSerialize for bedrock_core::types::uvar64 {
    fn serialize(&self) -> Result<Vec<u8>, SerilizationError>
    where
        Self: Sized,
    {
        let mut buf = vec![];

        match buf.write_u64_varint(self.0) {
            Ok(v) => v,
            Err(_) => return Err(SerilizationError::WriteVarintError),
        };

        Ok(buf)
    }
}

impl MCDeserialize for bedrock_core::types::uvar64 {
    fn deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized,
    {
        return Ok(Self(match data.read_u64_varint() {
            Ok(v) => v,
            Err(_) => return Err(DeserilizationError::ReadVarintError),
        }));
    }
}

// ivarint 64

impl MCSerialize for bedrock_core::types::ivar64 {
    fn serialize(&self) -> Result<Vec<u8>, SerilizationError>
    where
        Self: Sized,
    {
        let mut buf = vec![];

        match buf.write_i64_varint(self.0) {
            Ok(v) => v,
            Err(_) => return Err(SerilizationError::WriteVarintError),
        };

        Ok(buf)
    }
}

impl MCDeserialize for bedrock_core::types::ivar64 {
    fn deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized,
    {
        return Ok(Self(match data.read_i64_varint() {
            Ok(v) => v,
            Err(_) => return Err(DeserilizationError::ReadVarintError),
        }));
    }
}

// bool

impl MCSerialize for bool {
    fn serialize(&self) -> Result<Vec<u8>, SerilizationError>
    where
        Self: Sized,
    {
        Ok(match self {
            true => {
                vec![1]
            }
            false => {
                vec![0]
            }
        })
    }
}

impl MCDeserialize for bool {
    fn deserialize(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized,
    {
        if cursor.remaining() < 1 {
            return Err(DeserilizationError::NotEnoughtRemainingError);
        } else {
            return Ok(match cursor.get_u8() {
                0 => false,
                _ => true,
            });
        }
    }
}

// string

impl MCSerialize for String {
    fn serialize(&self) -> Result<Vec<u8>, SerilizationError>
    where
        Self: Sized,
    {
        let mut buf = vec![];

        buf.write_u64_varint(self.len() as u64);
        buf.put_slice(self.as_bytes());

        return Ok(buf);
    }
}

impl MCDeserialize for String {
    fn deserialize(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized,
    {
        let len = match cursor.read_u64_varint() {
            Ok(v) => v,
            Err(_) => return Err(DeserilizationError::ReadVarintError),
        };

        let mut string_buf = vec![0u8; len as usize];

        match cursor.read_exact(&mut *string_buf) {
            Ok(_) => {}
            Err(_) => return Err(DeserilizationError::NotEnoughtRemainingError),
        }

        match String::from_utf8(string_buf) {
            Ok(str) => Ok(str),
            Err(_) => Err(DeserilizationError::ReadUtf8StringError),
        }
    }
}
