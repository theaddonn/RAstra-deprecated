use crate::network::packet_info::{GamePacket, RAKNET_GAME_PACKET_ID};
use bytes::BufMut;
use varint_rs::VarintWriter;

pub struct PacketWriter {
    packet_game_type: GamePacket,
    buf: Vec<u8>,
}

impl PacketWriter {
    pub fn new_game_packet_writer(game_packet: GamePacket) -> Self {
        Self {
            packet_game_type: game_packet,
            buf: vec![],
        }
    }

    pub fn write_u8(&mut self, val: u8) {
        self.buf.put_u8(val)
    }
    pub fn write_u16(&mut self, val: u16) {
        self.buf.put_u16(val)
    }
    pub fn write_u32(&mut self, val: u32) {
        self.buf.put_u32(val)
    }
    pub fn write_u64(&mut self, val: u64) {
        self.buf.put_u64(val)
    }
    pub fn write_u128(&mut self, val: u128) {
        self.buf.put_u128(val)
    }

    pub fn write_i8(&mut self, val: i8) {
        self.buf.put_i8(val)
    }
    pub fn write_i16(&mut self, val: i16) {
        self.buf.put_i16(val)
    }
    pub fn write_i32(&mut self, val: i32) {
        self.buf.put_i32(val)
    }
    pub fn write_i64(&mut self, val: i64) {
        self.buf.put_i64(val)
    }
    pub fn write_i128(&mut self, val: i128) {
        self.buf.put_i128(val)
    }

    pub fn write_f32(&mut self, val: f32) {
        self.buf.put_f32(val)
    }
    pub fn write_f64(&mut self, val: f64) {
        self.buf.put_f64(val)
    }

    pub fn write_bool(&mut self, val: bool) {
        match val {
            false => self.buf.put_u8(0x00),
            true => self.buf.put_u8(0x01),
        }
    }

    pub fn write_uvarint(&mut self, val: usize) {
        self.buf.write_usize_varint(val).unwrap();
    }

    pub fn write_ivarint(&mut self, val: isize) {
        self.buf.write_isize_varint(val).unwrap();
    }

    pub fn get_payload(&self) -> Vec<u8> {
        // The buffer used for storing the finished payload
        let mut buf: Vec<u8> = vec![];

        // A temp buffer where we write the gamepacket id (bcz it's size in bytes is unknown)
        let mut gamepacket_id_buf: Vec<u8> = vec![];
        gamepacket_id_buf
            .write_usize_varint(self.packet_game_type.get_id())
            .unwrap();

        // Write the RakNet identifier for Gamepackets
        buf.put_u8(RAKNET_GAME_PACKET_ID);
        buf.write_usize_varint(self.buf.len() + gamepacket_id_buf.len())
            .unwrap();

        buf.put_slice(&*gamepacket_id_buf);
        buf.put_slice(&*self.buf);

        return buf;
    }

    pub fn get_raw_payload(&self) -> Vec<u8> {
        return self.buf.clone();
    }
}
