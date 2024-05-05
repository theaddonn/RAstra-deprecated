use crate::compression;
use crate::compression::CompressionMethod;
use crate::error::{CompressionError, ConnectionError};
use crate::gamepacket::GamePacket;
use crate::info::RAKNET_GAME_PACKET_ID;
use byteorder::{ReadBytesExt, WriteBytesExt};
use rak_rs::connection::queue::SendQueueError;
use rak_rs::connection::{Connection as RakConnection, RecvError};
use serialize::proto::ser::MCProtoSerialize;
use std::cell::Cell;
use std::io::{Cursor, Error, Write};
use varint_rs::{VarintReader, VarintWriter};

pub struct Connection {
    rak_connection: RakConnection,
    compression: Option<Box<dyn CompressionMethod>>,
    encryption: Option<Box<()>>,
}

impl Connection {
    pub fn new(conn: RakConnection) -> Self {
        Self {
            rak_connection: conn,
            compression: None,
            encryption: None,
        }
    }

    pub fn handle_login() {
        unimplemented!()
    }

    pub async fn send_gamepackets(
        &self,
        game_packets: Vec<GamePacket>,
    ) -> Result<(), ConnectionError> {
        let mut buf_pks = vec![];

        // Batch all gamepackets together
        for game_packet in game_packets {
            // Write gamepacket
            match game_packet.pk_serilize(&mut buf_pks) {
                Ok(_) => {}
                Err(e) => return Err(ConnectionError::SerializeError(e)),
            }
        }

        let mut buf = vec![];

        match buf.write_u8(RAKNET_GAME_PACKET_ID) {
            Ok(_) => {}
            Err(_) => return Err(ConnectionError::WriteIOError),
        };

        // Compress the data depending on compression method
        if let Some(compression) = &self.compression {
            // Write compression id
            match buf.write_u8(compression.get_IDu8()) {
                Ok(_) => {}
                Err(_) => return Err(ConnectionError::WriteIOError),
            };

            buf_pks = match compression.compress(&buf_pks) {
                Ok(v) => v,
                Err(e) => return Err(ConnectionError::CompressError(e)),
            };
        }

        // TODO: Encrypt the data (after compression)

        // Write final data into buf
        match buf.write_all(&*buf_pks) {
            Ok(_) => {}
            Err(_) => return Err(ConnectionError::WriteIOError),
        };

        match self.rak_connection.send(&*buf, true).await {
            Ok(_) => {}
            Err(_) => return Err(ConnectionError::RaknetError),
        }

        Ok(())
    }

    pub async fn recv_gamepackets(&mut self) -> Result<Vec<GamePacket>, ConnectionError> {
        // Recvieve data and turn it into cursor
        let mut data = match self.rak_connection.recv().await {
            Ok(v) => Cursor::new(v),
            Err(_) => return Err(ConnectionError::ConnectionClosed),
        };

        // Read Raknet Header
        match data.read_u8() {
            Ok(v) => {
                match v {
                    RAKNET_GAME_PACKET_ID => {}
                    // Invalid Raknet packet header
                    _ => return Err(ConnectionError::InvalidRaknetHeader),
                }
            }
            Err(_) => return Err(ConnectionError::ReadIOError),
        };

        // TODO: Decrypt the data (before decompression)

        // Decompress data depending on compression method
        if let Some(compression) = &self.compression {
            // Read compression algorythm
            match data.read_u8() {
                // If Compression algorythm id doesn't match
                Ok(v) => {
                    if !(v == compression.get_IDu8()) {
                        return Err(ConnectionError::InvalidCompressionMethod);
                    }
                }
                Err(_) => return Err(ConnectionError::ReadIOError),
            };

            let pos = data.position() as usize;

            data = match compression.decompress(&data.into_inner()[pos..].to_vec()) {
                Ok(v) => Cursor::new(v),
                Err(e) => return Err(ConnectionError::DecompressError(e)),
            };
        }

        'gamepacket_read: loop {
            let len = match data.read_u64_varint() {
                Ok(v) => v,
                Err(_) => return Err(ConnectionError::ReadIOError),
            };

            let id = match data.read_u64_varint() {
                Ok(v) => v,
                Err(_) => return Err(ConnectionError::ReadIOError),
            };


        }

        Ok(vec![])
    }
}
