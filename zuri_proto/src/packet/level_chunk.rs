use bytes::Bytes;
use glam::IVec2;
use crate::io::{Reader, Writer};
use crate::packet::PacketType;
use crate::types::world::SubChunkRequestMode;

#[derive(Debug)]
pub struct LevelChunk {
    pub position: IVec2,
    pub sub_chunk_request_mode: SubChunkRequestMode,
    pub highest_sub_chunk: u16,
    pub sub_chunk_count: u32,
    pub cache_enabled: bool,
    pub blob_hashes: Vec<u64>,
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
