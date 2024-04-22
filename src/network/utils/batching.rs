use std::io::{Cursor, Read};

use bytes::BufMut;
use varint_rs::VarintReader;

use crate::error::RastraError;

const MAXIMUM_BATCH_LENGTH: u64 = 512;

pub fn debatch_packetbatch(data: Vec<u8>) -> Result<Vec<Vec<u8>>, RastraError> {
    let mut packets: Vec<Vec<u8>> = vec![];
    let length = data.len() as u64;

    let mut cursor = Cursor::new(data.clone());

    while cursor.position() < length {
        if packets.len() as u64 > MAXIMUM_BATCH_LENGTH {
            return Err(RastraError::PacketBatchExceedsMaxError);
        }

        let len = match cursor.read_u64_varint() {
            Ok(val) => val as usize,
            Err(_) => {
                return Err(RastraError::ReadPacketBufferError);
            }
        };

        let mut packet_data = vec![0; len];
        if cursor.read_exact(&mut packet_data).is_err() {
            return Err(RastraError::ReadPacketBufferError);
        }

        packets.push(packet_data);
    }

    return Ok(packets);
}

pub fn batch_packetbatch(packets: Vec<Vec<u8>>) -> Vec<u8> {
    let mut buf = vec![];

    for packet in packets {
        buf.put_slice(&*packet);
    }

    buf
}
