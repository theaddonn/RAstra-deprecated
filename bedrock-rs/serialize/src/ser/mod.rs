use crate::error::SerilizationError;

pub trait MCSerialize {
    fn serialize(&self) -> Result<Vec<u8>, SerilizationError>
    where
        Self: Sized;
}
