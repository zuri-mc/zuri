use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Part of the blob cache protocol. It is sent by the client to let the server know what blobs it
/// needs and which blobs it already has, in an ACK type system.
#[derive(Debug, Clone)]
pub struct ClientCacheBlobStatus {
    /// A list of blob hashes that the client does not have a blob available for. The server should
    /// send the blobs matching these hashes as soon as possible.
    pub miss_hashes: Vec<u64>,
    /// A list of blob hashes that the client has a blob available for. The blobs hashes here mean
    /// that the client already has them: The server does not need to send the blobs anymore.
    pub hit_hashes: Vec<u64>,
}

impl PacketType for ClientCacheBlobStatus {
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
