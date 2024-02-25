use crate::error::RastraError;

pub trait GamepacketServer {
    fn serialize(&self) -> Result<&[u8], RastraError>;
}

pub trait GamepacketClient {
    fn deserialize(data: &[u8]) -> Result<Self, RastraError>
    where
        Self: Sized;
}

pub trait GamepacketBoth: GamepacketServer + GamepacketClient {}
