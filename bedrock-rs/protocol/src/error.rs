#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ProtocolError {
    ReadPacketBufError,
    ReadUtf8StringError,
}
