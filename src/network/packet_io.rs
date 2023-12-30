use crate::error::*;
use crate::utils::endian::Endian;
use bytes::{Buf, BufMut};
use std::{
    io::{Cursor, Read},
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr},
    str,
};
use tokio::io::AsyncWriteExt;
use varint::{VarintRead, VarintWrite};

pub struct PacketReader {
    buf: Cursor<Vec<u8>>,
}

impl PacketReader {
    pub fn new(buf: Vec<u8>) -> Self {
        Self {
            buf: Cursor::new(buf),
        }
    }
    pub fn read(&mut self, buf: &mut [u8]) -> ServerResult<()> {
        match self.buf.read_exact(buf) {
            Ok(p) => Ok(p),
            Err(_) => Err(Error::RAKNET_ReadPacketBufferError),
        }
    }
    pub fn read_u8(&mut self) -> ServerResult<u8> {
        Ok(self.buf.get_u8())
    }

    pub fn read_u16(&mut self, n: Endian) -> ServerResult<u16> {
        if self.buf.remaining() < 2 {
            return Err(Error::RAKNET_ReadPacketBufferError);
        }

        match n {
            Endian::Big => Ok(self.buf.get_u16()),
            Endian::Little => Ok(self.buf.get_u16_le()),
        }
    }

    pub fn read_u24(&mut self, n: Endian) -> ServerResult<u32> {
        if self.buf.remaining() < 3 {
            return Err(Error::RAKNET_ReadPacketBufferError);
        }

        match n {
            Endian::Big => {
                let a = self.buf.get_u8();
                let b = self.buf.get_u8();
                let c = self.buf.get_u8();

                let ret = u32::from_be_bytes([0, a, b, c]);
                Ok(ret)
            }
            Endian::Little => {
                let a = self.buf.get_u8();
                let b = self.buf.get_u8();
                let c = self.buf.get_u8();

                let ret = u32::from_le_bytes([a, b, c, 0]);
                Ok(ret)
            }
        }
    }

    pub fn read_u32(&mut self, n: Endian) -> ServerResult<u32> {
        if self.buf.remaining() < 4 {
            return Err(Error::RAKNET_ReadPacketBufferError);
        }

        match n {
            Endian::Big => Ok(self.buf.get_u32()),
            Endian::Little => Ok(self.buf.get_u32_le()),
        }
    }

    pub fn read_u64(&mut self, n: Endian) -> ServerResult<u64> {
        if self.buf.remaining() < 8 {
            return Err(Error::RAKNET_ReadPacketBufferError);
        }

        match n {
            Endian::Big => Ok(self.buf.get_u64()),
            Endian::Little => Ok(self.buf.get_u64_le()),
        }
    }
    pub fn read_i64(&mut self, n: Endian) -> ServerResult<i64> {
        if self.buf.remaining() < 8 {
            return Err(Error::RAKNET_ReadPacketBufferError);
        }

        match n {
            Endian::Big => Ok(self.buf.get_i64()),
            Endian::Little => Ok(self.buf.get_i64_le()),
        }
    }


    pub fn read_string(&mut self) -> ServerResult<String> {
        if self.buf.remaining() < 2 {
            return Err(Error::RAKNET_ReadPacketBufferError);
        }

        let size = self.read_u16(Endian::Big)?;
        let mut buf = vec![0u8; size as usize].into_boxed_slice();

        if self.buf.remaining() < size as usize {
            return Err(Error::RAKNET_ReadPacketBufferError);
        }

        self.read(&mut buf)?;
        Ok(String::from_utf8(buf.to_vec()).unwrap())
    }

    pub fn read_magic(&mut self) -> ServerResult<bool> {
        if self.buf.remaining() < 16 {
            return Err(Error::RAKNET_ReadPacketBufferError);
        }

        let mut magic = [0; 16];
        self.read(&mut magic)?;
        let offline_magic = [
            0x00, 0xff, 0xff, 0x00, 0xfe, 0xfe, 0xfe, 0xfe, 0xfd, 0xfd, 0xfd, 0xfd, 0x12, 0x34,
            0x56, 0x78,
        ];
        Ok(magic == offline_magic)
    }

    pub fn read_address(&mut self) -> ServerResult<SocketAddr> {
        let ip_ver = self.read_u8()?;

        if ip_ver == 4 {
            if self.buf.remaining() < 6 {
                return Err(Error::RAKNET_ReadPacketBufferError);
            }

            let ip = Ipv4Addr::new(
                0xff - self.read_u8()?,
                0xff - self.read_u8()?,
                0xff - self.read_u8()?,
                0xff - self.read_u8()?,
            );
            let port = self.read_u16(Endian::Big)?;
            Ok(SocketAddr::new(IpAddr::V4(ip), port))
        } else {
            if self.buf.remaining() < 44 {
                return Err(Error::RAKNET_ReadPacketBufferError);
            }

            self.next(2);
            let port = self.read_u16(Endian::Big)?;
            self.next(4);
            let mut addr_buf = [0; 16];
            self.read(&mut addr_buf)?;

            let mut address_cursor = PacketReader::new(addr_buf.to_vec());
            self.next(4);
            Ok(SocketAddr::new(
                IpAddr::V6(Ipv6Addr::new(
                    address_cursor.read_u16(Endian::Big)?,
                    address_cursor.read_u16(Endian::Big)?,
                    address_cursor.read_u16(Endian::Big)?,
                    address_cursor.read_u16(Endian::Big)?,
                    address_cursor.read_u16(Endian::Big)?,
                    address_cursor.read_u16(Endian::Big)?,
                    address_cursor.read_u16(Endian::Big)?,
                    address_cursor.read_u16(Endian::Big)?,
                )),
                port,
            ))
        } //IPv6 address = 128bit = u8 * 16
    }

    pub fn read_f32(&mut self) -> ServerResult<f32> {
        if self.buf.remaining() < 4 {
            return Err(Error::RAKNET_ReadPacketBufferError);
        }

        Ok(self.buf.get_f32())
    }

    pub fn read_f64(&mut self) -> ServerResult<f64> {
        if self.buf.remaining() < 8 {
            return Err(Error::RAKNET_ReadPacketBufferError);
        }

        Ok(self.buf.get_f64())
    }

    pub fn read_u32_varint(&mut self) -> ServerResult<u32> {
        if self.buf.remaining() < 2 {
            return Err(Error::RAKNET_ReadPacketBufferError);
        }

        match self.buf.read_unsigned_varint_32() {
            Ok(v) => Ok(v),
            Err(_) => Err(Error::RAKNET_ReadPacketBufferError)
        }
    }

    pub fn read_i32_varint(&mut self) -> ServerResult<i32> {
        if self.buf.remaining() < 2 {
            return Err(Error::RAKNET_ReadPacketBufferError);
        }

        match self.buf.read_signed_varint_32() {
            Ok(v) => Ok(v),
            Err(_) => Err(Error::RAKNET_ReadPacketBufferError)
        }
    }

    pub fn next(&mut self, n: u64) {
        self.buf.set_position(self.buf.position() + n);
    }

    pub fn pos(&self) -> u64 {
        self.buf.position()
    }
}

#[derive(Clone)]
pub struct PacketWriter {
    buf: Vec<u8>,
}

impl PacketWriter {
    pub fn new() -> Self {
        Self { buf: vec![] }
    }

    pub fn write(&mut self, v: &[u8]) -> ServerResult<()> {
        self.buf.put_slice(v);
        Ok(())
    }

    pub fn write_u8(&mut self, v: u8) -> ServerResult<()> {
        self.buf.put_u8(v);
        Ok(())
    }

    pub fn write_i16(&mut self, v: i16, n: Endian) -> ServerResult<()> {
        match n {
            Endian::Big => {
                self.buf.put_i16(v);
                Ok(())
            }
            Endian::Little => {
                self.buf.put_i16_le(v);
                Ok(())
            }
        }
    }

    pub fn write_u16(&mut self, v: u16, n: Endian) -> ServerResult<()> {
        match n {
            Endian::Big => {
                self.buf.put_u16(v);
                Ok(())
            }
            Endian::Little => {
                self.buf.put_u16_le(v);
                Ok(())
            }
        }
    }

    pub fn write_u24(&mut self, v: u32, n: Endian) -> ServerResult<()> {
        match n {
            Endian::Big => {
                let a = v.to_be_bytes();
                self.buf.put_u8(a[1]);
                self.buf.put_u8(a[2]);
                self.buf.put_u8(a[3]);
            }
            Endian::Little => {
                let a = v.to_le_bytes();
                self.buf.put_u8(a[0]);
                self.buf.put_u8(a[1]);
                self.buf.put_u8(a[2]);
            }
        }
        Ok(())
    }

    pub fn write_u32(&mut self, v: u32, n: Endian) -> ServerResult<()> {
        match n {
            Endian::Big => {
                self.buf.put_u32(v);
                Ok(())
            }
            Endian::Little => {
                self.buf.put_u32_le(v);
                Ok(())
            }
        }
    }

    pub fn write_i32(&mut self, v: i32, n: Endian) -> ServerResult<()> {
        match n {
            Endian::Big => {
                self.buf.put_i32(v);
                Ok(())
            }
            Endian::Little => {
                self.buf.put_i32_le(v);
                Ok(())
            }
        }
    }

    pub fn write_i64(&mut self, v: i64, n: Endian) -> ServerResult<()> {
        match n {
            Endian::Big => {
                self.buf.put_i64(v);
                Ok(())
            }
            Endian::Little => {
                self.buf.put_i64_le(v);
                Ok(())
            }
        }
    }

    pub fn write_magic(&mut self) -> ServerResult<usize> {
        let magic: [u8; 16] = [
            0x00, 0xff, 0xff, 0x00, 0xfe, 0xfe, 0xfe, 0xfe, 0xfd, 0xfd, 0xfd, 0xfd, 0x12, 0x34,
            0x56, 0x78,
        ];
        self.buf.put_slice(&magic);
        Ok(magic.len())
    }

    pub fn write_u64(&mut self, v: u64, n: Endian) -> ServerResult<()> {
        match n {
            Endian::Big => {
                self.buf.put_u64(v);
                Ok(())
            }
            Endian::Little => {
                self.buf.put_u64_le(v);
                Ok(())
            }
        }
    }

    pub fn write_string(&mut self, body: &str) -> ServerResult<()> {
        let raw = body.as_bytes();

        match self.write_u16(raw.len() as u16, Endian::Big) {
            Ok(_) => {
                self.buf.put_slice(&raw);
                Ok(())
            }
            Err(err) => { Err(err)}
        }
    }

    pub fn write_address(&mut self, address: SocketAddr) -> ServerResult<()> {
        if address.is_ipv4() {
            self.write_u8(0x4)?;
            let ip_bytes = match address.ip() {
                IpAddr::V4(ip) => ip.octets().to_vec(),
                _ => vec![0; 4],
            };

            self.write_u8(0xff - ip_bytes[0])?;
            self.write_u8(0xff - ip_bytes[1])?;
            self.write_u8(0xff - ip_bytes[2])?;
            self.write_u8(0xff - ip_bytes[3])?;
            self.write_u16(address.port(), Endian::Big)?;
            Ok(())
        } else {
            self.write_i16(23, Endian::Little)?;
            self.write_u16(address.port(), Endian::Big)?;
            self.write_i32(0, Endian::Big)?;
            let ip_bytes = match address.ip() {
                IpAddr::V6(ip) => ip.octets().to_vec(),
                _ => vec![0; 16],
            };
            self.write(&ip_bytes)?;
            self.write_i32(0, Endian::Big)?;
            Ok(())
        }
    }

    pub fn write_f32(&mut self, val: f32) -> ServerResult<()> {
        self.buf.put_f32(val);
        Ok(())
    }

    pub fn write_f64(&mut self, val: f64) -> ServerResult<()> {
        self.buf.put_f64(val);
        Ok(())
    }

    pub fn write_u32_varint(&mut self, val: u32) -> ServerResult<()> {
        let mut data = Cursor::new(vec![]);
        match data.write_unsigned_varint_32(val) {
            Ok(_) => {
                match self.buf.write(&*data.get_mut()) {
                    Ok(_) => Ok(()),
                    Err(_) => Err(Error::PACKET_WRITER_InavlidVarint)
                }
            }
            Err(_) => Err(Error::PACKET_WRITER_InavlidVarint)
        }
    }

    pub fn write_i32_varint(&mut self, val: i32) -> ServerResult<()> {
        let mut data = Cursor::new(vec![]);
        match data.write_signed_varint_32(val) {
            Ok(_) => {
                match self.buf.write(&*data.get_mut()) {
                    Ok(_) => Ok(()),
                    Err(_) => Err(Error::PACKET_WRITER_InavlidVarint)
                }
            }
            Err(_) => Err(Error::PACKET_WRITER_InavlidVarint)
        }
    }

    pub fn get_raw_payload(self) -> Vec<u8> {
        self.buf
    }

    pub fn _pos(&self) -> u64 {
        self.buf.len() as u64
    }
}
