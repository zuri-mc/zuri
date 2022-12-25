use zuri_proto::io::Writer;
use bytes::{Bytes, BytesMut};

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

    pub fn encode(&mut self, batch: &mut Vec<Bytes>) -> Result<Bytes, String> {
        let mut batch_writer = Writer::new(0);
        for packet in batch {
            batch_writer.byte_slice(&packet);
        }

        let mut batch: BytesMut = batch_writer.into();
        if let Some(compression) = &mut self.compression {
            match compression.compress(batch.into()) {
                Ok(v) => batch = BytesMut::from(v.as_slice()), // TODO: IS THERE A BETTER WAY?
                Err(s) => return Err(s),
            }
        }
        if let Some(encryption) = &mut self.encryption {
            batch = BytesMut::from(encryption.encrypt(batch.into()).as_slice()); // TODO: IS THERE A BETTER WAY?
        }

        // TODO: IS THERE A BETTER WAY?
        BytesMut::from(&[PACKET_HEADER][..]).unsplit(batch);

        Ok(batch.into())
    }
}
