pub trait PacketEncode {
    fn encode(&self) -> [u8];
}
