use bytes::Bytes;

use zuri_net_derive::proto;

/// Sent to the client so that the client can download the resource pack. Each packet holds a chunk
/// of the compressed resource pack, of which the size is defined in the ResourcePackDataInfo packet
/// sent before.
#[proto]
#[derive(Debug, Clone)]
pub struct ResourcePackChunkData {
    /// The unique ID of the resource pack that the chunk of data is taken out of.
    pub uuid: String,
    /// The current chunk index of the chunk. It is a number that starts at 0 and is incremented for
    /// each resource pack data chunk sent to the client.
    pub chunk_index: u32,
    /// The current progress in bytes or offset in the data that the resource pack data chunk is
    /// taken from.
    pub data_offset: u64,
    /// Byte slice containing a chunk of data from the resource pack. It must be of the same size or
    /// less than the `data_chunk_size` set in the ResourcePackDataInfo packet.
    pub data: Bytes,
}
