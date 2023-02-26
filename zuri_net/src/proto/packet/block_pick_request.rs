use zuri_net_derive::proto;

use crate::proto::io::BlockPos;

/// Sent by the client when it requests to pick a block in the world and place its item in their
/// inventory.
#[proto]
#[derive(Debug, Clone)]
pub struct BlockPickRequest {
    /// The position at which the client requested to pick the block. The block at that position
    /// should have its item put in HotBarSlot if it is empty.
    pub position: BlockPos,
    /// Specifies if the item should get all NBT tags from the block, meaning the item places a
    /// block practically always equal to the one picked.
    pub add_block_nbt: bool,
    /// The slot that was held at the time of picking a block.
    pub hotbar_slot: u8,
}
