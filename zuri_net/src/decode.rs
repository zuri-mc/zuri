use bytes::Bytes;
use zuri_proto::io::Reader;

use crate::encryption::Encryption;
use crate::compression::Compression;

#[derive(Default)]
pub struct Decoder {
    compression: Option<Compression>,
    encryption: Option<Encryption>,
}

/// The header used for all compressed 'batches' in Minecraft.
const PACKET_HEADER: u8 = 0xfe;

/// The maximum amount of packets that can be sent in a single batch.
const MAXIMUM_IN_BATCH: usize = 512 + 256;

impl Decoder {
    pub fn set_compression(&mut self, compression: Option<Compression>) {
        self.compression = compression;
    }

    pub fn set_encryption(&mut self, encryption: Option<Encryption>) {
        self.encryption = encryption;
    }

    pub fn decode(&mut self, batch: &mut Vec<u8>) -> Result<Vec<Vec<u8>>, String> {
        if batch.is_empty() {
            return Ok(Vec::new());
        }
        if batch[0] != PACKET_HEADER {
            return Err(format!("invalid packet header (expected {}, got {})", PACKET_HEADER, batch[0]))?;
        }

        // TODO: IS THERE A BETTER WAY TO AVOID SLICE(..)

        batch.remove(0);
        if let Some(encryption) = &mut self.encryption {
            if let Err(s) = encryption.decrypt(batch) {
                return Err(s);
            }
        }
        if let Some(compression) = &mut self.compression {
            if let Err(s) =  compression.decompress(batch) {
                return Err(s);
            }
        }

        let mut packets = Vec::new();
        let mut batch_reader = Reader::from_buf(Bytes::from(batch.clone()), 0);
        while batch.len() > 0 {
            packets.push(batch_reader.byte_slice().to_vec());
        }

        if packets.len() > MAXIMUM_IN_BATCH {
            Err(format!("too many packets in batch ({} > {})", packets.len(), MAXIMUM_IN_BATCH))?
        }
        Ok(packets)
    }
}
