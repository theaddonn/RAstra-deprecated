#[derive(Copy, Clone, Debug)]
pub enum SerilizationError {
    WriteVarintError,
}

#[derive(Copy, Clone, Debug)]
pub enum DeserilizationError {
    NotEnoughtRemainingError,
    ReadVarintError,
    ReadUtf8StringError
}
