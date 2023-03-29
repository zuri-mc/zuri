use crate::proto::ints::VarU32;
use glam::IVec2;
use zuri_net_derive::proto;

use crate::proto::io::BlockPos;

/// Sent by the server to change the point around which chunks are and remain loaded. This is useful
/// for mini-game servers, where only one area is ever loaded, in which case the
/// NetworkChunkPublisherUpdate packet can be sent in the middle of it, so that no chunks ever need
/// to be additionally sent during the course of the game. In reality, the packet is not
/// extraordinarily useful, and most servers just send it constantly at the position of the player.
/// If the packet is not sent at all, no chunks will be shown to the player, regardless of where
/// they are sent.
#[proto]
#[derive(Debug, Clone)]
pub struct NetworkChunkPublisherUpdate {
    /// The block position around which chunks loaded will remain shown to the client. Most servers
    /// set this position to the position of the player itself.
    pub position: BlockPos,
    /// The radius in blocks around Position that chunks sent show up in and will remain loaded in.
    /// Unlike the RequestChunkRadius and ChunkRadiusUpdated packets, this radius is in blocks
    /// rather than chunks, so the chunk radius needs to be multiplied by sixteen. (Or shifted to
    /// the left by four.)
    pub radius: VarU32,
    /// It is unclear what the purpose of this field is.
    #[len_type(u32)]
    pub saved_chunks: Vec<IVec2>,
}
