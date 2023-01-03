use crate::proto::packet::PacketType;
use crate::proto::io::{Reader, Writer};
use crate::proto::types::world::CacheBlob;

/// Part of the blob cache protocol. It is sent by the server in response to a ClientCacheBlobStatus
/// packet and contains the blob data of all blobs that the client acknowledged not to have yet.
#[derive(Debug, Clone)]
pub struct ClientCacheMissResponse {
    /// A list of all blobs that the client sent misses for in the ClientCacheBlobStatus. These
    /// blobs hold the data of the blobs with the hashes they are matched with.
    pub blobs: Vec<CacheBlob>,
}

impl PacketType for ClientCacheMissResponse {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.blobs.len() as u32);
        self.blobs.iter().for_each(|blob| blob.write(writer));
    }

    fn read(reader: &mut Reader) -> Self {
        Self { blobs: (0..reader.var_u32()).map(|_| CacheBlob::read(reader)).collect() }
    }
}
