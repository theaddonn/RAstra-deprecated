use bytes::BufMut;

use crate::error::RastraError;
use crate::error::RastraError::RaknetPacketIdError;
use crate::network::conn_info::ConnInfo;
use crate::network::utils::batching::{batch_packetbatch, debatch_packetbatch};
use crate::network::utils::compression::{compress, decompress};
use crate::network::utils::encryption::{decrypt, encrypt};
use crate::utils::compression_method::CompressionMethod;

const RAKNET_GAMEPACKET_ID: u8 = 0xfe;

pub async fn decode(mut data: Vec<u8>, conn_info: &ConnInfo) -> Result<Vec<Vec<u8>>, RastraError> {
    if data[0] == RAKNET_GAMEPACKET_ID {
        data.remove(0);
    } else {
        return Err(RaknetPacketIdError);
    }

    let mut decompressed_data = if conn_info.compressed {
        let compression_type = data[0] as i8;
        data.remove(0);

        match compression_type {
            -1 => data,
            0 => match decompress(data, &CompressionMethod::Flate) {
                Ok(v) => v,
                Err(e) => return Err(e),
            },
            1 => match decompress(data, &CompressionMethod::Snappy) {
                Ok(v) => v,
                Err(e) => return Err(e),
            },
            _ => return Err(RastraError::CompressionMethodUnknown),
        }
    } else {
        data
    };

    let decrypted_data = if conn_info.encrypted {
        decrypt(decompressed_data, conn_info)
    } else {
        decompressed_data
    };

    let buf = match debatch_packetbatch(decrypted_data) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    Ok(buf)
}

pub async fn encode(packets: Vec<Vec<u8>>, conn_info: &mut ConnInfo) -> Result<Vec<u8>, RastraError> {
    let mut buf = vec![];

    buf.put_u8(RAKNET_GAMEPACKET_ID);

    if conn_info.compressed {
        buf.put_i8(conn_info.compression_method.ok_or(RastraError::CompressionMethodUnknown).unwrap().to_int() as i8);
    }

    let data = batch_packetbatch(packets);

    let data = if conn_info.encrypted {
        encrypt(data, conn_info)
    } else {
        data
    };

    let data = if conn_info.compressed {
        match compress(data, &conn_info.compression_method.unwrap()) {
            Ok(v) => v,
            Err(e) => return Err(e),
        }
    } else {
        data
    };

    buf.put_slice(&*data);

    Ok(buf)
}
