use std::io::Cursor;
use crate::error::ProtocolError;

pub trait Serialize {
    fn serialize(&self) -> Result<Vec<u8>, ProtocolError> where Self: Sized;
}

pub trait Deserialize {
    fn deserialize(data: &mut Cursor<Vec<u8>>) -> Result<Self, ProtocolError> where Self: Sized;
}

pub trait GamepacketSerialize {
    fn to_packet(&self) -> Result<Vec<u8>, ProtocolError> where Self: Sized + Serialize;
}

pub trait GamepacketDeserialize {
    fn from_packet(data: Vec<u8>) -> Result<Self, ProtocolError> where Self: Sized + Deserialize;
}

