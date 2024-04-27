use std::io::{Cursor, Error, Read};
use std::string::FromUtf8Error;

use bytes::{Buf, BufMut};
use varint_rs::{VarintReader, VarintWriter};

use serialize::de::MCDeserialize;
use serialize::error::{DeserilizationError, SerilizationError};
use serialize::ser::MCSerialize;

#[derive(Debug)]
pub struct ConnectionRequestType {
    /// Array of Base64 encoded JSON Web Token certificates to authenticate the player.
    /// The last certificate in the chain will have a property 'extraData' that contains player identity information including the XBL XUID (if the player was signed into XBL at the time of the connection).
    pub certificate_chain: String,
    /// Base64 encoded JSON Web Token that contains other relevant client properties.
    /// Properties Include:
    /// - SelfSignedId
    /// - ServerAddress = (unresolved url if applicable)
    /// - ClientRandomId
    /// - SkinId
    /// - SkinData
    /// - SkinImageWidth
    /// - SkinImageHeight
    /// - CapeData
    /// - CapeImageWidth
    /// - CapeImageHeight
    /// - SkinResourcePatch
    /// - SkinGeometryData
    /// - SkinGeometryDataEngineVersion
    /// - SkinAnimationData
    /// - PlayFabId
    /// - AnimatedImageData = Array of:
    ///   - Type
    ///   - Image
    ///   - ImageWidth
    ///   - ImageHeight
    ///   - Frames
    ///   - AnimationExpression
    /// - ArmSize
    /// - SkinColor
    /// - PersonaPieces = Array of:
    ///   - PackId
    ///   - PieceId
    ///   - IsDefault
    ///   - PieceType
    ///   - ProuctId
    /// - PieceTintColors = Array of:
    ///   - PieceType
    ///   - Colors = Array of color hexstrings
    /// - IsEduMode (if edu mode)
    /// - TenantId (if edu mode)
    /// - ADRole (if edu mode)
    /// - IsEditorMode
    /// - GameVersion
    /// - DeviceModel
    /// - DeviceOS = (see enumeration: BuildPlatform)
    /// - DefaultInputMode = (see enumeration: InputMode)
    /// - CurrentInputMode = (see enumeration: InputMode)
    /// - UIProfile = (see enumeration: UIProfile)
    /// - GuiScale
    /// - LanguageCode
    /// - PlatformUserId
    /// - ThirdPartyName
    /// - ThirdPartyNameOnly
    /// - PlatformOnlineId
    /// - PlatformOfflineId
    /// - DeviceId
    /// - TrustedSkin
    /// - PremiumSkin
    /// - PersonaSkin
    /// - OverrideSkin
    /// - CapeOnClassicSkin
    /// - CapeId
    /// - CompatibleWithClientSideChunkGen
    pub raw_token: String,
}

impl MCSerialize for ConnectionRequestType {
    fn serialize(&self) -> Result<Vec<u8>, SerilizationError>
    where
        Self: Sized,
    {
        let mut buf = vec![];

        // Write entire length
        // 8 = i32 + i32 for length of both
        match buf.write_u64_varint((self.certificate_chain.len() + self.certificate_chain.len() + 8) as u64) {
            Ok(_) => {}
            Err(_) => { return Err(SerilizationError::WriteVarintError) }
        };

        // write length of certificate_chain vec
        buf.put_i32_le(self.certificate_chain.len() as i32);

        // write strings (certificate_chain)
        buf.put_slice(&self.certificate_chain.as_bytes());

        // write length of raw_token vec
        buf.put_i32_le(self.raw_token.len() as i32);

        // write strings (raw_token)
        buf.put_slice(&self.raw_token.as_bytes());

        Ok(buf)
    }
}

impl MCDeserialize for ConnectionRequestType {
    fn deserialize(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized,
    {

        // read the ConnectionRequest's length
        // (certificate_chain len + raw_token len + 8)
        // 8 = i32 len + i32 len (length of certificate_chain's len and raw_token's len)
        // can be ignored, other lengths are provided
        match cursor.read_u64_varint() {
            Ok(_) => {},
            Err(_) => return Err(DeserilizationError::ReadVarintError),
        };

        // read length of certificate_chain vec
        let certificate_chain_len = cursor.get_i32_le();

        println!("certificate_chain_len: {certificate_chain_len}");

        let mut certificate_chain_buf = vec![0; certificate_chain_len as usize];

        // read string data (certificate_chain)
        match cursor.read_exact(&mut certificate_chain_buf) {
            Ok(_) => {}
            Err(_) => { return Err(DeserilizationError::NotEnoughtRemainingError) }
        };

        // transform into string
        let certificate_chain = match String::from_utf8(certificate_chain_buf) {
            Ok(v) => { v }
            Err(_) => { return Err(DeserilizationError::ReadUtf8StringError) }
        };

        // read length of certificate_chain vec
        let raw_token_len = cursor.get_i32_le();

        println!("raw_token_len: {raw_token_len}");

        let mut raw_token_buf = vec![0; raw_token_len as usize];

        // read string data (certificate_chain)
        match cursor.read_exact(&mut raw_token_buf) {
            Ok(_) => {}
            Err(_) => { return Err(DeserilizationError::NotEnoughtRemainingError) }
        };

        // transform into string
        let raw_token = match String::from_utf8(raw_token_buf) {
            Ok(v) => { v }
            Err(_) => { return Err(DeserilizationError::ReadUtf8StringError) }
        };

        return Ok(Self {
            certificate_chain,
            raw_token,
        });
    }
}
