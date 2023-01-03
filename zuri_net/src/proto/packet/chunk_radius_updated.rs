use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent by the server in response to a RequestChunkRadius packet. It defines the chunk radius that
/// the server allows the client to have. This may be lower than the chunk radius requested by the
/// client in the RequestChunkRadius packet.
#[derive(Debug, Clone)]
pub struct ChunkRadiusUpdated {
    /// The final chunk radius that the client will adapt when it receives the packet. It does not
    /// have to be the same as the requested chunk radius.
    pub chunk_radius: i32,
}

impl PacketType for ChunkRadiusUpdated {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.chunk_radius);
    }

    fn read(reader: &mut Reader) -> Self {
        Self { chunk_radius: reader.var_i32() }
    }
}
