use std::collections::btree_map::Values;
use std::collections::BTreeMap;
use std::io::{Cursor, Error, Read};
use std::string::FromUtf8Error;
use std::iter::Map;

use bytes::{Buf, BufMut};
use serde_json::Value;
use varint_rs::{VarintReader, VarintWriter};

use serialize::de::MCDeserialize;
use serialize::error::{DeserilizationError, SerilizationError};
use serialize::ser::MCSerialize;

#[derive(Debug)]
pub struct ConnectionRequestType {
    /// Array of Base64 encoded JSON Web Token certificates to authenticate the player.
    ///
    /// The last certificate in the chain will have a property 'extraData' that contains player identity information including the XBL XUID (if the player was signed into XBL at the time of the connection).
    pub certificate_chain: Vec<BTreeMap<String, Value>>,
    /// Base64 encoded JSON Web Token that contains other relevant client properties.
    ///
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
    pub raw_token: BTreeMap<String, Value>,
}

// TODO: Add MCSerialize
impl MCSerialize for ConnectionRequestType {
    fn serialize(&self) -> Result<Vec<u8>, SerilizationError>
    where
        Self: Sized,
    {
        let mut buf = vec![];

        // Write entire length
        // 8 = i32 + i32 for length of both
        //match buf.write_u64_varint((self.certificate_chain.len() + self.certificate_chain.len() + 8) as u64) {
        //    Ok(_) => {}
        //    Err(_) => { return Err(SerilizationError::WriteVarintError) }
        //};

        // write length of certificate_chain vec
        // buf.put_i32_le(self.certificate_chain.len() as i32);

        // write strings (certificate_chain)
        //buf.put_slice(&self.certificate_chain.as_bytes());

        // write length of raw_token vec
        // buf.put_i32_le(self.raw_token.len() as i32);

        // write strings (raw_token)
        //buf.put_slice(&self.raw_token.as_bytes());

        Ok(buf)
    }
}

// TODO: Add microsoft auth
impl MCDeserialize for ConnectionRequestType {
    fn deserialize(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized,
    {
        let mut certificate_chain: Vec<BTreeMap<String, Value>> = vec![];

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

        let mut certificate_chain_buf = vec![0; certificate_chain_len as usize];

        // read string data (certificate_chain)
        match cursor.read_exact(&mut certificate_chain_buf) {
            Ok(_) => {}
            Err(_) => { return Err(DeserilizationError::NotEnoughtRemainingError) }
        };

        // transform into string
        let certificate_chain_string = match String::from_utf8(certificate_chain_buf) {
            Ok(v) => { v }
            Err(_) => { return Err(DeserilizationError::ReadUtf8StringError) }
        };

        // parse certificate chain string into json
        let certificate_chain_json = match serde_json::from_str(certificate_chain_string.as_str()) {
            Ok(v) => { v },
            Err(_) => { return Err(DeserilizationError::ReadJsonError) }
        };

        let certificate_chain_json_jwts = match certificate_chain_json {
            Value::Object(mut v) => {
                match v.get_mut("chain") {
                    None => {
                        // the certificate chain should always be a object with just an array of jwts called "chain"
                        return Err(DeserilizationError::ReadJsonError)
                    }
                    Some(v) => {
                        match v.take() {
                            Value::Array(v) => { v }
                            _ => {
                                // the certificate chain should always be a object with just an array of jwts called "chain"
                                return Err(DeserilizationError::ReadJsonError)
                            }
                        }
                    }
                }
            }
            o => {
                // the certificate chain should always be a object with just an array of jwts called "chain"
                return Err(DeserilizationError::ReadJsonError);
            }
        };

        for jwt_json in certificate_chain_json_jwts {
            let jwt_string = match jwt_json {
                Value::String(str) => { str },
                o => {
                    // the certificate chain's should always be a jwt string
                    return Err(DeserilizationError::ReadJsonError);
                }
            };

            // Decode the jwt string into a jwt
            let jwt = match jwtk::decode_without_verify::<BTreeMap<String, Value>>(jwt_string.as_str()) {
                Ok(v) => { v },
                Err(_) => { return Err(DeserilizationError::ReadJwtError) }
            };

            certificate_chain.push(jwt.claims().extra.to_owned());
        }

        // read length of certificate_chain vec
        let raw_token_len = cursor.get_i32_le();

        let mut raw_token_buf = vec![0; raw_token_len as usize];

        // read string data (certificate_chain)
        match cursor.read_exact(&mut raw_token_buf) {
            Ok(_) => {}
            Err(_) => { return Err(DeserilizationError::NotEnoughtRemainingError) }
        };

        // transform into string
        let raw_token_string = match String::from_utf8(raw_token_buf) {
            Ok(v) => { v }
            Err(_) => { return Err(DeserilizationError::ReadUtf8StringError) }
        };

        let raw_token = match jwtk::decode_without_verify::<BTreeMap<String, Value>>(raw_token_string.as_str()) {
            Ok(v) => { v.claims().extra.clone() },
            Err(_) => {
                return Err(DeserilizationError::ReadJwtError)
            }
        };

        return Ok(Self {
            certificate_chain,
            raw_token,
        });
    }
}
