use bytes::Bytes;
use zuri_net_derive::proto;

use crate::proto::types::resource_pack::ResourcePackType;

/// Sent by the server to the client to inform the client about the data contained in one of the
/// resource packs that are about to be sent.
#[proto]
#[derive(Debug, Clone)]
pub struct ResourcePackDataInfo {
    /// The unique ID of the resource pack that the info concerns.
    pub uuid: String,
    /// The maximum size in bytes of the chunks in which the total size of the resource pack to be
    /// sent will be divided. A size of 1MB (1024*1024) means that a resource pack of 15.5MB will be
    /// split into 16 data chunks.
    pub data_chunk_size: u32,
    /// The total amount of data chunks that the sent resource pack will exist out of. It is the
    /// total size of the resource pack divided by the `data_chunk_size` field. The client doesn't
    /// actually seem to use this field. Rather, it divides the size by the chunk size to calculate
    /// it itself.
    pub chunk_count: u32,
    /// The total size in bytes that the resource pack occupies. This is the size of the compressed
    /// archive (zip) of the resource pack.
    pub size: u64,
    /// SHA256 hash of the content of the resource pack.
    pub hash: Bytes,
    /// Specifies if the resource pack was a premium resource pack, meaning it was bought from the
    /// Minecraft store.
    pub premium: bool,
    /// The type of the resource pack.
    pub pack_type: ResourcePackType,
}
