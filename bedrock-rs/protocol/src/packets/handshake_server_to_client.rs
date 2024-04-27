use std::collections::BTreeMap;
use std::io::Cursor;

use bytes::BufMut;
use jwtk::{
    ecdsa::{EcdsaAlgorithm, EcdsaPrivateKey, EcdsaPublicKey},
    sign,
};
use serde_json::Value;
use varint_rs::VarintWriter;

use serialize::de::MCDeserialize;
use serialize::error::{DeserilizationError, SerilizationError};
use serialize::ser::MCSerialize;

#[derive(Debug)]
pub struct HandshakeServerToClientPacket {
    pub handshake_webtoken: BTreeMap<String, Value>,
    pub public_key_x5u: EcdsaPublicKey,
}

impl MCSerialize for HandshakeServerToClientPacket {
    fn serialize(&self) -> Result<Vec<u8>, SerilizationError>
    where
        Self: Sized,
    {
        let key = match EcdsaPrivateKey::generate(EcdsaAlgorithm::ES384) {
            Ok(v) => v,
            Err(_) => {
                return Err(SerilizationError::GenerateKeyError);
            }
        };

        let mut jwt = jwtk::HeaderAndClaims::new_dynamic();

        jwt.header_mut();

        for (string, val) in self.handshake_webtoken.iter() {
            jwt.insert(string, val.clone());
        }

        let token = match sign(&mut jwt, &key) {
            Ok(v) => v,
            Err(_) => {
                return Err(SerilizationError::JwtError);
            }
        };

        println!("token:\n{}", token);

        let mut buf = vec![];

        match buf.write_u64_varint(token.len() as u64) {
            Ok(_) => {}
            Err(_) => {
                return Err(SerilizationError::WriteVarintError);
            }
        }

        buf.put_slice(token.as_bytes());

        Ok(buf)
    }
}

impl MCDeserialize for HandshakeServerToClientPacket {
    fn deserialize(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized,
    {
        todo!()
    }
}
