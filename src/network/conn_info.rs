use crate::error::RastraError;
use crate::utils::compression_method::CompressionMethod;

pub struct ConnInfo {
    pub compressed: bool,
    pub encrypted: bool,
    pub send_counter: u64,
    pub key_bytes: Vec<u8>,
    pub stream: Option<Box<dyn crypto::cipher::StreamCipher + Send + Sync>>,
    pub compression_method: Option<CompressionMethod>,
}

impl ConnInfo {
    pub fn new() -> Self {
        Self {
            compressed: false,
            encrypted: false,
            send_counter: 0,
            key_bytes: vec![],
            stream: None,
            compression_method: None,
        }
    }

    pub fn enable_compression(&mut self, compression_method: CompressionMethod) {
        self.compressed = true;
        self.compression_method = Some(compression_method);
    }

    pub fn enable_encryption(&mut self, key_bytes: [u8; 32]) -> Result<(), RastraError> {
        //let block = match Aes128::new_from_slice(&key_bytes) {
        //    Ok(v) => { v }
        //    Err(_) => { return Err(RastraError::RecvError); }
        //};
        //
        //let mut first_12 = vec![0u8; 12];
        //first_12.copy_from_slice(&key_bytes[..12]);
        //
        //let mut nonce = [0u8; 16];
        //nonce[..12].copy_from_slice(&first_12);
        //nonce[15] = 2;
        //
        //
        //let cipher = match Ctr128LE::new_from_slices(&block.into(), &nonce.into()) {
        //    Ok(v) => { v }
        //    Err(_) => { return Err(RastraError::SendError); }
        //};
        //
        //self.encrypted = true;
        //self.stream = Some(Box::new(cipher));

        Ok(())
    }
}
