#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ProtocolError {
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ListenerError {
    AddrBindErr,
}

#[derive(Debug)]
pub enum CompressionError {
    ZlibError(flate2::CompressError),
    SnappyError(snap::Error)
}

#[derive(Debug)]
pub enum DecompressionError {
    ZlibError(flate2::DecompressError),
    SnappyError(snap::Error)
}