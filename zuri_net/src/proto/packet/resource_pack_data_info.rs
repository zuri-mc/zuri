use bytes::Bytes;
use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;
use crate::proto::types::resource_pack::ResourcePackType;

/// Sent by the server to the client to inform the client about the data contained in one of the
/// resource packs that are about to be sent.
#[derive(Debug, Clone)]
pub struct ResourcePackDataInfo {
    /// The unique ID of the resource pack that the info concerns.
    pub uuid: String,
    /// The maximum size in bytes of the chunks in which the total size of the resource pack to be
    /// sent will be divided. A size of 1MB (1024*1024) means that a resource pack of 15.5MB will be
    /// split into 16 data chunks.
    pub data_chunk_size: u32,
    /// The total amount of data chunks that the sent resource pack will exist out of. It is the
    /// total size of the resource pack divided by the `data_chunk_size` field. The client doesn't
    /// actually seem to use this field. Rather, it divides the size by the chunk size to calculate
    /// it itself.
    pub chunk_count: u32,
    /// The total size in bytes that the resource pack occupies. This is the size of the compressed
    /// archive (zip) of the resource pack.
    pub size: u64,
    /// SHA256 hash of the content of the resource pack.
    pub hash: Bytes,
    /// Specifies if the resource pack was a premium resource pack, meaning it was bought from the
    /// Minecraft store.
    pub premium: bool,
    /// The type of the resource pack.
    pub pack_type: ResourcePackType,
}

impl PacketType for ResourcePackDataInfo {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.uuid.as_str());
        writer.u32(self.data_chunk_size);
        writer.u32(self.chunk_count);
        writer.u64(self.size);
        writer.byte_slice(&self.hash);
        writer.bool(self.premium);
        writer.u8(self.pack_type.to_u8().unwrap());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            uuid: reader.string(),
            data_chunk_size: reader.u32(),
            chunk_count: reader.u32(),
            size: reader.u64(),
            hash: reader.byte_slice(),
            premium: reader.bool(),
            pack_type: ResourcePackType::from_u8(reader.u8()).unwrap(),
        }
    }
}
