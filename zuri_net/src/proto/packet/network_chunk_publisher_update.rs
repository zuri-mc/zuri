use glam::{IVec2, IVec3};

use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent by the server to change the point around which chunks are and remain loaded. This is useful
/// for mini-game servers, where only one area is ever loaded, in which case the
/// NetworkChunkPublisherUpdate packet can be sent in the middle of it, so that no chunks ever need
/// to be additionally sent during the course of the game. In reality, the packet is not
/// extraordinarily useful, and most servers just send it constantly at the position of the player.
/// If the packet is not sent at all, no chunks will be shown to the player, regardless of where
/// they are sent.
#[derive(Debug, Clone)]
pub struct NetworkChunkPublisherUpdate {
    /// The block position around which chunks loaded will remain shown to the client. Most servers
    /// set this position to the position of the player itself.
    pub position: IVec3,
    /// The radius in blocks around Position that chunks sent show up in and will remain loaded in.
    /// Unlike the RequestChunkRadius and ChunkRadiusUpdated packets, this radius is in blocks
    /// rather than chunks, so the chunk radius needs to be multiplied by sixteen. (Or shifted to
    /// the left by four.)
    pub radius: u32,
    /// It is unclear what the purpose of this field is.
    pub saved_chunks: Vec<IVec2>,
}

impl PacketType for NetworkChunkPublisherUpdate {
    fn write(&self, writer: &mut Writer) {
        writer.block_pos(self.position);
        writer.var_u32(self.radius);
        writer.u32(self.saved_chunks.len() as u32);
        self.saved_chunks.iter().for_each(|chunk| {
            writer.var_i32(chunk.x);
            writer.var_i32(chunk.y);
        });
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            position: reader.block_pos(),
            radius: reader.var_u32(),
            saved_chunks: (0..reader.u32()).map(|_| IVec2::new(reader.var_i32(), reader.var_i32())).collect(),
        }
    }
}
