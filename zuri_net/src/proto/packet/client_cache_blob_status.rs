use zuri_net_derive::packet;

use crate::proto::ints::VarU32;

/// Part of the blob cache protocol. It is sent by the client to let the server know what blobs it
/// needs and which blobs it already has, in an ACK type system.
#[packet]
#[derive(Debug, Clone)]
pub struct ClientCacheBlobStatus {
    #[size_for(miss_hashes)]
    __: VarU32,
    #[size_for(hit_hashes)]
    __: VarU32,
    /// A list of blob hashes that the client does not have a blob available for. The server should
    /// send the blobs matching these hashes as soon as possible.
    pub miss_hashes: Vec<u64>,
    /// A list of blob hashes that the client has a blob available for. The blobs hashes here mean
    /// that the client already has them: The server does not need to send the blobs anymore.
    pub hit_hashes: Vec<u64>,
}
