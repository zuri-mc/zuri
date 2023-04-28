use zuri_net_derive::proto;

use crate::proto::ints::VarI32;

/// Sent by the client to the server to update the server on the chunk view radius that it has set
/// in the settings. The server may respond with a ChunkRadiusUpdated packet with either the chunk
/// radius requested, or a different chunk radius if the server chooses so.
#[proto]
#[derive(Debug, Clone)]
pub struct RequestChunkRadius {
    /// The requested chunk radius. This value is the value set in the settings of the player.
    pub chunk_radius: VarI32,
    /// The maximum chunk radius that the player wants to receive.
    pub max_chunk_radius: VarI32,
}
