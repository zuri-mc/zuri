use zuri_net_derive::proto;

/// Sent by the client to request a chunk of data from a particular resource pack, that it has
/// obtained information about in a ResourcePackDataInfo packet.
#[proto]
#[derive(Debug, Clone)]
pub struct ResourcePackChunkRequest {
    /// The unique ID of the resource pack that the chunk of data is requested from.
    pub uuid: String,
    /// The requested chunk index of the chunk. It is a number that starts at zero and is
    /// incremented for each resource pack data chunk requested.
    pub chunk_index: u32,
}
