use serialize::error::{DeserilizationError, SerilizationError};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ProtocolError {}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ListenerError {
    AddrBindError,
    AlreadyOnline,
    NotListening,
}

#[derive(Debug)]
pub enum ConnectionError {
    ReadIOError,
    WriteIOError,
    SerializeError(SerilizationError),
    DeserializeError(DeserilizationError),
    ConnectionClosed,
    RaknetError,
    CompressError(CompressionError),
    DecompressError(DecompressionError),
    InvalidRaknetHeader,
    InvalidCompressionMethod,
}

#[derive(Debug)]
pub enum CompressionError {
    ZlibError(flate2::CompressError),
    SnappyError(snap::Error),
}

#[derive(Debug)]
pub enum DecompressionError {
    ZlibError(flate2::DecompressError),
    SnappyError(snap::Error),
}
