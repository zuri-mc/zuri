use glam::IVec3;
use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent by the client when it requests to pick a block in the world and place its item in their inventory.
#[derive(Debug)]
pub struct BlockPickRequest {
    /// The position at which the client requested to pick the block. The block at that position should have its item
    /// put in HotBarSlot if it is empty.
    pub position: IVec3,
    /// Specifies if the item should get all NBT tags from the block, meaning the item places a block practically
    /// always equal to the one picked.
    pub add_block_nbt: bool,
    /// The slot that was held at the time of picking a block.
    pub hotbar_slot: u8,
}

impl PacketType for BlockPickRequest {
    fn write(&self, writer: &mut Writer) {
        writer.block_pos(self.position);
        writer.bool(self.add_block_nbt);
        writer.u8(self.hotbar_slot);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            position: reader.block_pos(),
            add_block_nbt: reader.bool(),
            hotbar_slot: reader.u8(),
        }
    }
}
