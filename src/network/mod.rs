use crate::network::packet_io::PacketReader;
use crate::network::packet_io::PacketWriter;
use crate::utils::endian::Endian;

pub mod packet;
pub mod packet_io;
pub mod packet_io_game;

#[tokio::test]
async fn test_u24_encode_decode() {
    let a: u32 = 65535 * 21;
    let b = a.to_le_bytes();
    let mut reader = PacketReader::new(b.to_vec());

    let c = reader.read_u24(Endian::Little).unwrap();

    assert!(a == c);

    let mut writer = PacketWriter::new();
    writer.write_u24(a, Endian::Little).unwrap();

    let buf = writer.get_raw_payload();
    let mut reader = PacketReader::new(buf);

    let c = reader.read_u24(Endian::Little).unwrap();

    assert!(a == c);
}
