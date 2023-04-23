use bytes::Bytes;
use glam::IVec2;

use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;
use crate::proto::types::world::SubChunkRequestMode;

/// Sent by the server to provide the client with a chunk of a world data (16xYx16 blocks).
/// Typically, a certain amount of chunks is sent to the client before sending it the spawn
/// PlayStatus packet, so that the client spawns in a loaded world.
#[derive(Debug, Clone)]
pub struct LevelChunk {
    /// The X and Z coordinates of the chunk sent. You can convert a block coordinate to a chunk
    /// coordinate by right-shifting it four bits.
    pub position: IVec2,
    /// Specifies the mode in which chunks are sent. If this is anything but legacy, the sub-chunk
    /// request system is used.
    pub sub_chunk_request_mode: SubChunkRequestMode,
    /// The highest sub-chunk at the position that is not all air. It is only set if the sub
    /// chunk count is set to limited.
    pub highest_sub_chunk: u16,
    /// The amount of sub-chunks that are part of the chunk sent. Depending on if the cache is
    /// enabled, a list of blob hashes will be sent, or, if disabled, the sub-chunk data.
    pub sub_chunk_count: u32,
    /// Specifies if the client blob cache should be enabled. This system is based on hashes of
    /// blobs which are consistent and saved by the client in combination with that blob, so that
    /// the server does not have the same chunk multiple times. If the client does not yet have a
    /// blob with the hash sent, it will send a ClientCacheBlobStatus packet containing the hashes
    /// it does not have the data of.
    pub cache_enabled: bool,
    /// A list of all blob hashes used in the chunk. It is composed of `sub_chunk_count + 1` hashes,
    /// with the first SubChunkCount hashes being those of the sub-chunks and the last one that of
    /// the biome of the chunk. If caching is not enabled, this can be left empty.
    pub blob_hashes: Vec<u64>,
    /// A serialised string of chunk data. The data held depends on if CacheEnabled is set to true.
    /// If set to false, the payload is composed of multiple sub-chunks, each of which carry a
    /// version which indicates the way they are serialised, followed by biomes, border blocks and
    /// tile entities. If caching is enabled, the payload consists out of the border blocks and tile
    /// entities only.
    pub raw_payload: Bytes,
}

impl PacketType for LevelChunk {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.position.x);
        writer.var_i32(self.position.y);
        match self.sub_chunk_request_mode {
            SubChunkRequestMode::Legacy => {
                writer.var_u32(self.sub_chunk_count);
            }
            SubChunkRequestMode::Limitless => {
                writer.var_u32(u32::MAX);
            }
            SubChunkRequestMode::Limited => {
                writer.var_u32(u32::MAX - 1);
                writer.u16(self.highest_sub_chunk);
            }
        }
        writer.bool(self.cache_enabled);
        if self.cache_enabled {
            writer.var_u32(self.blob_hashes.len() as u32);
            self.blob_hashes.iter().for_each(|hash| writer.u64(*hash));
        }
        writer.byte_slice(&self.raw_payload);
    }

    fn read(reader: &mut Reader) -> Self {
        let mut packet = Self {
            position: IVec2::new(reader.var_i32(), reader.var_i32()),
            sub_chunk_request_mode: SubChunkRequestMode::Legacy,
            highest_sub_chunk: 0,
            sub_chunk_count: 0,
            cache_enabled: false,
            blob_hashes: Vec::new(),
            raw_payload: Bytes::default(),
        };
        let sub_chunk_count = reader.var_u32();
        if sub_chunk_count == u32::MAX {
            packet.sub_chunk_request_mode = SubChunkRequestMode::Limitless;
        } else if sub_chunk_count == u32::MAX - 1 {
            packet.sub_chunk_request_mode = SubChunkRequestMode::Limited;
            packet.highest_sub_chunk = reader.u16();
        } else {
            packet.sub_chunk_count = sub_chunk_count;
        }
        packet.cache_enabled = reader.bool();
        if packet.cache_enabled {
            let blob_hashes_len = reader.var_u32() as usize;
            packet.blob_hashes = (0..blob_hashes_len).map(|_| reader.u64()).collect();
        }
        packet.raw_payload = reader.byte_slice();

        packet
    }
}
