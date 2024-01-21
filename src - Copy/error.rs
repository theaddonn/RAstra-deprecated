#[derive(Debug)]
pub enum Error {
    RAKNET_SetRaknetRawSocketError,
    RAKNET_NotListen,
    RAKNET_BindAdressError,
    RAKNET_ConnectionClosed,
    RAKNET_NotSupportVersion,
    RAKNET_IncorrectReply,
    RAKNET_PacketParseError,
    RAKNET_SocketError,
    RAKNET_IncorrectReliability,
    RAKNET_IncorrectPacketID,
    RAKNET_ReadPacketBufferError,
    RAKNET_PacketSizeExceedMTU,
    RAKNET_PacketHeaderError,
    SERVER_GamepacketIDUnknown,
    SERVER_DEVICEOSIDUnknown,
    SERVER_ArmSizeUnknown,
    SERVER_ColorHexInvalid,
    SERVER_PlayerOffline,
    PACKET_WRITER_InavlidVarint,
}

pub type ServerResult<T> = Result<T, Error>;
