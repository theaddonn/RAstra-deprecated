use bedrock_rs::protocol::info::GamePacketType;
use ecdsa::elliptic_curve::pkcs8::LineEnding;
use ecdsa::elliptic_curve::pkcs8::LineEnding::CRLF;
use jsonwebtoken::{EncodingKey, Header};
use p384::ecdsa::{SigningKey, VerifyingKey};
use p384::pkcs8::der::{Decode, Encode};
use p384::pkcs8::{Document, EncodePrivateKey, EncodePublicKey};
use p384::SecretKey;
use rak_rs::connection::RecvError;
use serde_json::json;
use signature::rand_core::OsRng;

use crate::error::RastraError;
use crate::network::conn_info::ConnInfo;
use crate::network::connection::connection::{ConnectionReadHalf, ConnectionWriteHalf};
use crate::network::info::{GamePacket, MOJANG_JWT_ALG, MOJANG_PUBLIC_JWT_KEY};
use crate::network::utils::handler::encode;
use crate::{log_info, log_warning};

pub async fn handle_handshake(
    connection_read_half: &mut ConnectionReadHalf,
    connection_write_half: &ConnectionWriteHalf,
    conn_info: &mut ConnInfo,
) -> Result<(), RastraError> {
    //let secret_key = SigningKey::random(&mut OsRng);

    let ecdh_private_key = SecretKey::random(&mut OsRng);

    let ecdh_public_key = ecdh_private_key.public_key();

    let private_key_pem = ecdh_private_key.to_sec1_pem(LineEnding::CR).unwrap();
    let public_key_der = ecdh_public_key.to_public_key_der().unwrap();

    let client_x509 = base64::encode(&public_key_der);

    let key = EncodingKey::from_ec_der(public_key_der.as_bytes());

    let mut header = Header::new(MOJANG_JWT_ALG);

    header.x5u = Some(client_x509.clone());

    let claims = json!({
        "salt": "7efOy9ye/iNLxLjj2m5YQw==",
        "signedToken": client_x509
    });

    let jwt_token = match jsonwebtoken::encode(&header, &claims, &key) {
        Ok(v) => v,
        Err(e) => {
            log_info!(e);
            return Err(RastraError::JWTError);
        }
    };

    let mut pk_writer = PacketWriter::new_game_packet_writer();

    pk_writer.write_string(jwt_token);

    let data = pk_writer.get_payload(GamePacketType::ServerToClientHandshake);

    let ser_data = encode(vec![data], conn_info).await.unwrap();

    connection_write_half.send(&*ser_data, true).await.unwrap();

    let data = connection_read_half.recv().await;

    match data {
        Ok(v) => {
            log_info!(format!("success: {:?}", v))
        }
        Err(_) => {
            log_warning!("recv not successful ")
        }
    }

    Ok(())
}
