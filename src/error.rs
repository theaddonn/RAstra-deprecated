use std::fmt::Debug;

#[derive(Debug)]
#[allow(non_snake_case)]
pub enum RastraError {
    ServerGamepacketUnknown,
    ServerDeviceOsUnknown,
    ServerArmSizeUnknown,
    ServerColorHexInvalid,
    ServerPlayerNotConnected,

    PacketWriterInvalidVarint,
}
