#[derive(Copy, Clone, Debug)]
pub enum SerilizationError {
    WriteVarintError,
    GenerateKeyError,
    JwtError,
}

#[derive(Copy, Clone, Debug)]
pub enum DeserilizationError {
    NotEnoughtRemainingError,
    ReadVarintError,
    ReadUtf8StringError,
    ReadJsonError,
    ReadJwtError,
}
