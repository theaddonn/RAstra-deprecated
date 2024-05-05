#[derive(Copy, Clone, Debug)]
pub enum SerilizationError {
    WriteIOError,
    GenerateKeyError,
    JwtError,
}

#[derive(Copy, Clone, Debug)]
pub enum DeserilizationError {
    ReadIOError,
    NotEnoughtRemainingError,
    ReadUtf8StringError,
    ReadJsonError,
    ReadJwtError,
    InvalidGamepacketID
}
