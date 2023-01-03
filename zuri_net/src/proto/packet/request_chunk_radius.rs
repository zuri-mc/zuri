use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent by the client to the server to update the server on the chunk view radius that it has set
/// in the settings. The server may respond with a ChunkRadiusUpdated packet with either the chunk
/// radius requested, or a different chunk radius if the server chooses so.
#[derive(Debug, Clone)]
pub struct RequestChunkRadius {
    /// The requested chunk radius. This value is the value set in the settings of the player.
    pub chunk_radius: i32,
}

impl PacketType for RequestChunkRadius {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.chunk_radius);
    }

    fn read(reader: &mut Reader) -> Self {
        Self { chunk_radius: reader.var_i32() }
    }
}
