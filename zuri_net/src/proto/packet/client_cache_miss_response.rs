use crate::proto::packet::PacketType;
use crate::proto::io::{Reader, Writer};
use crate::proto::types::world::CacheBlob;

#[derive(Debug, Clone)]
pub struct ClientCacheMissResponse {
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
