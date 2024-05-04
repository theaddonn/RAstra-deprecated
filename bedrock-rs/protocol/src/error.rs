#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ProtocolError {
    ReadPacketBufError,
    ReadUtf8StringError,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ListenerError {
    AddrBindErr
}