use std::fmt::Debug;

#[derive(Debug)]
pub enum RastraError {
    GamepacketUnknown,
    DeviceOsUnknown,
    ArmSizeUnknown,
    CompressionMethodUnknown,
    ColorHexInvalid,

    CliCommandNameAlreadyTaken,

    CouldNotSerialize,
    CouldNotDeserialize,

    ReadPacketBufferError,
    DecompressionError,
    CompressionError,
    RaknetPacketIdError,
    PacketBatchExceedsMaxError,
    RecvError,
    SendError,

    PlayerNotOnlineError,
    WrongProtocolVersionError,
    UTF8StringError,
    JWTError,
    EcdsaKeyError
}
