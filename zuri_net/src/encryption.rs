use aes::Aes256;
use sha2::Digest;
use bytes::BufMut;
use cipher::{InnerIvInit, KeyInit, StreamCipherCore};

type Ctr128BE<Cipher> = ctr::CtrCore<Cipher, ctr::flavors::Ctr128BE>;

pub struct Encryption {
    send_counter: u64,
    key: Vec<u8>,
    cipher: Box<ctr::CtrCore<Aes256, ctr::flavors::Ctr128BE>>,
}

impl Encryption {
    pub fn new(key: Vec<u8>) -> Self {
        let mut iv = key.clone();
        iv.truncate(12);
        iv.extend_from_slice(&[0, 0, 0, 2]);

        let cipher = Box::new(Aes256::new_from_slice(&key)
            .and_then(|aes| Ctr128BE::inner_iv_slice_init(aes, &iv))
            .unwrap());

        Self {
            send_counter: 0,
            cipher,
            key,
        }
    }

    pub fn encrypt(&mut self, data: &mut Vec<u8>) {
        let mut send_buf = Vec::new();
        send_buf.put_u64_le(self.send_counter);
        self.send_counter += 1;

        let mut digest = sha2::Sha256::new();
        digest.update(&send_buf);
        digest.update(&data);
        digest.update(&self.key);

        data.append(&mut digest.finalize()[0..8].to_vec());

        self.cipher.clone().apply_keystream_partial(data.as_mut_slice().into());
    }

    pub fn decrypt(&mut self, data: &mut Vec<u8>) -> Result<(), String> {
        self.cipher.clone().apply_keystream_partial(data.as_mut_slice().into());
        if data.len() < 8 {
            Err("encrypted packet must be at least 8 bytes long")?
        }

        let their_checksum: Vec<u8> = data.iter().rev().take(8).rev().cloned().collect();

        let mut send_buf = Vec::new();
        send_buf.put_u64_le(self.send_counter);
        self.send_counter += 1;

        data.truncate(data.len() - 8);

        let mut digest = sha2::Sha256::new();
        digest.update(&send_buf);
        digest.update(&data);
        digest.update(&self.key);

        let our_checksum = digest.finalize()[0..8].to_vec();
        if their_checksum != our_checksum {
            Err(format!("invalid checksum (expected {:?}, got {:?})", our_checksum, their_checksum))?
        }

        Ok(())
    }
}

