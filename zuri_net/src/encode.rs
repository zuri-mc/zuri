use std::collections::VecDeque;
use zuri_proto::io::Writer;

use crate::encryption::Encryption;
use crate::compression::Compression;

#[derive(Default)]
pub struct Encoder {
    compression: Option<Compression>,
    encryption: Option<Encryption>,
}

/// The header used for all compressed 'batches' in Minecraft.
const PACKET_HEADER: u8 = 0xfe;

impl Encoder {
    pub fn set_compression(&mut self, compression: Option<Compression>) {
        self.compression = compression;
    }

    pub fn set_encryption(&mut self, encryption: Option<Encryption>) {
        self.encryption = encryption;
    }

    pub fn encode(&mut self, batch: &mut Vec<Vec<u8>>) -> Result<Vec<u8>, String> {
        let mut batch_writer = Writer::new(0);
        for packet in batch {
            batch_writer.byte_slice(&packet);
        }

        let mut batch: Vec<u8> = batch_writer.into();
        if let Some(compression) = &mut self.compression {
            if let Err(s) = compression.compress(&mut batch) {
                return Err(s);
            }
        }
        if let Some(encryption) = &mut self.encryption {
            encryption.encrypt(&mut batch);
        }
        batch.insert(0, PACKET_HEADER);

        Ok(batch.into())
    }
}
