use zuri_net_derive::proto;

use crate::proto::ints::VarI32;

/// Sent by the server in response to a RequestChunkRadius packet. It defines the chunk radius that
/// the server allows the client to have. This may be lower than the chunk radius requested by the
/// client in the RequestChunkRadius packet.
#[proto]
#[derive(Debug, Clone)]
pub struct ChunkRadiusUpdated {
    /// The final chunk radius that the client will adapt when it receives the packet. It does not
    /// have to be the same as the requested chunk radius.
    pub chunk_radius: VarI32,
}
