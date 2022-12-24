use crate::io::{Reader, Writer};
use crate::packet::Packet;

#[derive(Debug)]
pub struct ClientCacheBlobStatus {
    pub miss_hashes: Vec<u64>,
    pub hit_hashes: Vec<u64>,
}

impl Packet for ClientCacheBlobStatus {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.miss_hashes.len() as u32);
        writer.var_u32(self.hit_hashes.len() as u32);
        self.miss_hashes.iter().for_each(|hash| writer.u64(*hash));
        self.hit_hashes.iter().for_each(|hash| writer.u64(*hash));
    }

    fn read(reader: &mut Reader) -> Self {
        let miss_hashes_len = reader.var_u32();
        let hit_hashes_len = reader.var_u32();
        Self {
            miss_hashes: (0..miss_hashes_len).map(|_| reader.u64()).collect(),
            hit_hashes: (0..hit_hashes_len).map(|_| reader.u64()).collect(),
        }
    }
}
