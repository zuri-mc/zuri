use bytes::Bytes;

use crate::compression::Compression;
use crate::encryption::Encryption;
use crate::proto::io::{Reader, Writer};

#[derive(Default)]
pub struct Encoder {
    compression: Option<Compression>,
    encryption: Option<Encryption>,
}

/// The header used for all 'batches' in Minecraft.
const PACKET_HEADER: u8 = 0xfe;

/// The maximum amount of packets that can be sent in a single batch.
const MAXIMUM_IN_BATCH: usize = 512 + 256;

impl Encoder {
    pub fn set_compression(&mut self, compression: Compression) {
        self.compression = Some(compression);
    }

    pub fn set_encryption(&mut self, encryption: Encryption) {
        self.encryption = Some(encryption);
    }

    pub fn encode(&mut self, batch: &mut Vec<Vec<u8>>) -> Result<Vec<u8>, String> {
        let mut batch_writer = Writer::new(0);
        for packet in batch {
            batch_writer.byte_slice(&packet);
        }

        let mut batch: Vec<u8> = batch_writer.into();
        if let Some(compression) = &mut self.compression {
            compression.compress(&mut batch)?;
        }
        if let Some(encryption) = &mut self.encryption {
            encryption.encrypt(&mut batch);
        }
        batch.insert(0, PACKET_HEADER);

        Ok(batch)
    }

    pub fn decode(&mut self, batch: &mut Vec<u8>) -> Result<Vec<Vec<u8>>, String> {
        if batch.is_empty() {
            return Err(format!("expected populated batch, got empty batch"));
        }
        if batch[0] != PACKET_HEADER {
            return Err(format!(
                "invalid packet header (expected {}, got {})",
                PACKET_HEADER, batch[0]
            ))?;
        }

        batch.remove(0);
        if let Some(encryption) = &mut self.encryption {
            encryption.decrypt(batch)?;
        }
        if let Some(compression) = &mut self.compression {
            compression.decompress(batch)?;
        }

        let mut packets = Vec::new();
        let mut batch_reader = Reader::from_buf(Bytes::from(batch.clone()), 0);
        while batch_reader.len() > 0 {
            packets.push(batch_reader.byte_slice().to_vec());
        }

        if packets.len() > MAXIMUM_IN_BATCH {
            Err(format!(
                "too many packets in batch ({} > {})",
                packets.len(),
                MAXIMUM_IN_BATCH
            ))?
        }
        Ok(packets)
    }
}
