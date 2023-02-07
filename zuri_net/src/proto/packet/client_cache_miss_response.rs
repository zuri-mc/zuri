use zuri_net_derive::proto;

use crate::proto::ints::VarU32;
use crate::proto::types::world::CacheBlob;

/// Part of the blob cache protocol. It is sent by the server in response to a ClientCacheBlobStatus
/// packet and contains the blob data of all blobs that the client acknowledged not to have yet.
#[proto]
#[derive(Debug, Clone)]
pub struct ClientCacheMissResponse {
    /// A list of all blobs that the client sent misses for in the ClientCacheBlobStatus. These
    /// blobs hold the data of the blobs with the hashes they are matched with.
    #[len_type(VarU32)]
    pub blobs: Vec<CacheBlob>,
}
