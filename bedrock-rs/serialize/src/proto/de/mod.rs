use std::io::Cursor;

use crate::proto::error::DeserilizationError;

///
pub trait MCProtoDeserialize {
    #[inline]
    fn proto_deserialize(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized;
}