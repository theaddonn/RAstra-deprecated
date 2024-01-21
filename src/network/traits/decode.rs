pub trait PacketDecode {
    fn decode(buf: [u8]) -> Self;
}
