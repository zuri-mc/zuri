use aes::Aes256;
use bytes::BufMut;
use cipher::{KeyIvInit, StreamCipher};
use sha2::Digest;

type Aes256Ctr = ctr::Ctr32BE<Aes256>;

pub struct Encryption {
    sent: u64,
    read: u64,

    cipher: Aes256Ctr,
    decipher: Aes256Ctr,

    key: Vec<u8>,
}

impl Encryption {
    pub fn new(key: Vec<u8>) -> Self {
        let mut iv = key.clone();
        iv.truncate(12);
        iv.extend_from_slice(&[0, 0, 0, 2]);

        let base_cipher = Aes256Ctr::new(key.as_slice().into(), iv.as_slice().into());
        Self {
            sent: 0,
            read: 0,

            cipher: base_cipher.clone(),
            decipher: base_cipher,

            key,
        }
    }

    pub fn encrypt(&mut self, data: &mut Vec<u8>) {
        let mut send_buf = Vec::new();
        send_buf.put_u64_le(self.sent);
        self.sent += 1;

        let mut digest = sha2::Sha256::new();
        digest.update(&send_buf);
        digest.update(&data);
        digest.update(&self.key);

        let mut our_checksum = digest.finalize()[0..8].to_vec();
        data.append(&mut our_checksum);

        self.cipher.apply_keystream(data);
    }

    pub fn decrypt(&mut self, data: &mut Vec<u8>) -> Result<(), String> {
        self.decipher.apply_keystream(data);
        if data.len() < 8 {
            return Err("encrypted packet must be at least 8 bytes long")?;
        }

        let their_checksum: Vec<u8> = data.iter().rev().take(8).rev().cloned().collect();

        let mut send_buf = Vec::new();
        send_buf.put_u64_le(self.read);
        self.read += 1;

        data.truncate(data.len() - 8);

        let mut digest = sha2::Sha256::new();
        digest.update(&send_buf);
        digest.update(&data);
        digest.update(&self.key);

        let our_checksum = digest.finalize()[0..8].to_vec();
        if their_checksum != our_checksum {
            return Err(format!("invalid checksum (expected {:?}, got {:?})", our_checksum, their_checksum))?;
        }

        Ok(())
    }
}
