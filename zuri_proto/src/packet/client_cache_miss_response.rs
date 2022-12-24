use crate::io::{Reader, Writer};
use crate::packet::Packet;

#[derive(Debug)]
pub struct ClientCacheMissResponse {
    pub blobs: Vec<CacheBlob>,
}

impl Packet for ClientCacheMissResponse {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.blobs.len() as u32);
        self.blobs.iter().for_each(|blob| blob.write(writer));
    }

    fn read(reader: &mut Reader) -> Self {
        Self { blobs: (0..reader.var_u32()).map(|_| CacheBlob::read(reader)).collect() }
    }
}
