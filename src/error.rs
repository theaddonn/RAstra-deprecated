use std::fmt::Debug;

#[derive(Debug)]
pub enum RastraError {
    ServerGamepacketUnknown,
    ServerDeviceOsUnknown,
    ServerArmSizeUnknown,
    ServerColorHexInvalid,
    ServerPlayerNotConnected,

    PacketWriterInvalidVarint,

    CliCommandNameAlreadyTaken
}
