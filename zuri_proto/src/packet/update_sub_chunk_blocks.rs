use glam::IVec3;
use crate::io::{Reader, Writer};
use crate::packet::Packet;

#[derive(Debug)]
pub struct UpdateSubChunkBlocks {
    pub position: IVec3,
    pub blocks: Vec<BlockChangeEntry>,
    pub extra: Vec<BlockChangeEntry>,
}

impl Packet for UpdateSubChunkBlocks {
    fn write(&self, writer: &mut Writer) {
        writer.block_pos(self.position);
        writer.var_u32(self.blocks.len() as u32);
        self.blocks.iter().for_each(|entry| entry.write(writer));
        writer.var_u32(self.extra.len() as u32);
        self.extra.iter().for_each(|entry| entry.write(writer));
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            position: reader.block_pos(),
            blocks: (0..reader.var_u32()).map(|_| BlockChangeEntry::read(reader)).collect(),
            extra: (0..reader.var_u32()).map(|_| BlockChangeEntry::read(reader)).collect(),
        }
    }
}

#[derive(Debug)]
pub struct BlockChangeEntry {
    pub block_pos: IVec3,
    pub block_runtime_id: u32,
    pub flags: u32,
    pub synced_update_entity_unique_id: u64,
    pub synced_update_type: u32,
}

impl BlockChangeEntry {
    pub fn write(&self, writer: &mut Writer) {
        writer.block_pos(self.block_pos);
        writer.var_u32(self.block_runtime_id);
        writer.var_u32(self.flags);
        writer.var_u64(self.synced_update_entity_unique_id);
        writer.var_u32(self.synced_update_type);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            block_pos: reader.block_pos(),
            block_runtime_id: reader.var_u32(),
            flags: reader.var_u32(),
            synced_update_entity_unique_id: reader.var_u64(),
            synced_update_type: reader.var_u32(),
        }
    }
}

