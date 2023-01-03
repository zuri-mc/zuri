use glam::IVec3;
use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::packet::PacketType;
use crate::proto::io::{Reader, Writer};
use crate::proto::types::world::UpdateBlockTransition;

/// Essentially just the UpdateBlock packet, however for a set of blocks in a sub-chunk.
#[derive(Debug, Clone)]
pub struct UpdateSubChunkBlocks {
    /// The position of the sub-chunk being referred to.
    pub position: IVec3,
    /// Each updated block change entry.
    pub blocks: Vec<BlockChangeEntry>,
    /// Each updated block change entry for the second layer, usually for waterlogged blocks.
    pub extra: Vec<BlockChangeEntry>,
}

impl PacketType for UpdateSubChunkBlocks {
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

/// Used by the UpdateSubChunkBlocks packet to specify a block change entry.
#[derive(Debug, Clone)]
pub struct BlockChangeEntry {
    /// The position of the block being changed.
    pub block_pos: IVec3,
    /// The runtime ID of the block.
    pub block_runtime_id: u32,
    /// A combination of flags that specify the way the block is updated client-side.
    pub flags: u32,
    /// The unique ID of the falling block entity that the block transitions to or that the entity
    /// transitions from. Note that for both possible values for TransitionType, the
    /// `entity_unique_id` should point to the falling block entity involved.
    pub synced_update_entity_unique_id: u64,
    /// The type of the transition that happened. It is either `BlockToEntity`, when a block placed
    /// becomes a falling entity, or `EntityToBlock`, when a falling entity hits the ground and
    /// becomes a solid block again.
    pub synced_update_type: UpdateBlockTransition,
}

impl BlockChangeEntry {
    pub fn write(&self, writer: &mut Writer) {
        writer.block_pos(self.block_pos);
        writer.var_u32(self.block_runtime_id);
        writer.var_u32(self.flags);
        writer.var_u64(self.synced_update_entity_unique_id);
        writer.var_u32(self.synced_update_type.to_u32().unwrap());
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            block_pos: reader.block_pos(),
            block_runtime_id: reader.var_u32(),
            flags: reader.var_u32(),
            synced_update_entity_unique_id: reader.var_u64(),
            synced_update_type: UpdateBlockTransition::from_u32(reader.var_u32()).unwrap(),
        }
    }
}
