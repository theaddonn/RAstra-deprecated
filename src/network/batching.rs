use std::io::{Cursor, Read};

use bytes::Buf;
use varint_rs::VarintReader;

use crate::error::RastraError;

pub fn debatch_packet_stream(data: Vec<u8>) -> Vec<Result<Vec<u8>, RastraError>> {
    let mut packets: Vec<Result<Vec<u8>, RastraError>> = vec![];

    let mut cursor = Cursor::new(data.clone());
    cursor.get_u8();

    let mut last_end = 0;

    'packet_reader: while last_end < data.len() {
        let len = match cursor.read_u64_varint() {
            Ok(val) => val as usize,
            Err(err) => {
                packets.push(Err(RastraError::ReadPacketBufferError));
                continue 'packet_reader;
            }
        };

        let mut packet_data = vec![0; len];
        if cursor.read_exact(&mut packet_data).is_err() {
            packets.push(Err(RastraError::ReadPacketBufferError));
            continue 'packet_reader;
        }

        packets.push(Ok(packet_data));
        last_end = cursor.position() as usize; // Update last_end here
    }

    return packets;
}
