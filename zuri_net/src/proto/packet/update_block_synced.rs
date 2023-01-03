use glam::IVec3;
use num_traits::{FromPrimitive, ToPrimitive};
use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;
use crate::proto::types::world::UpdateBlockTransition;

/// Sent by the server to synchronise the falling of a falling block entity with the transitioning
/// back and forth from and to a solid block. It is used to prevent the entity from flickering, and
/// is used in places such as the pushing of blocks with pistons.
#[derive(Debug, Clone)]
pub struct UpdateBlockSynced {
    /// The block position at which a block is updated.
    pub position: IVec3,
    /// The runtime ID of the new block that is placed at position.
    pub new_block_runtime_id: u32,
    /// A combination of `BlockUpdate` flags that specify the way the block is updated client-side.
    /// Typically, sending only the `Network` flag is sufficient.
    pub flags: u32,
    /// The world layer on which the block is updated. For most blocks, this is the first layer, as
    /// that layer is the default layer to place blocks on.
    pub layer: u32,
    /// The unique ID of the falling block entity that the block transitions to or that the entity
    /// transitions from. Note that for both possible values for TransitionType, the
    /// `entity_unique_id` should point to the falling block entity involved.
    pub entity_unique_id: i64,
    /// The type of the transition that happened. It is either `BlockToEntity`, when a block placed
    /// becomes a falling entity, or `EntityToBlock`, when a falling entity hits the ground and
    /// becomes a solid block again.
    pub transition_type: UpdateBlockTransition,
}

impl PacketType for UpdateBlockSynced {
    fn write(&self, writer: &mut Writer) {
        writer.u_block_pos(self.position);
        writer.var_u32(self.new_block_runtime_id);
        writer.var_u32(self.flags);
        writer.var_u32(self.layer);
        writer.var_i64(self.entity_unique_id);
        writer.var_u64(self.transition_type.to_u64().unwrap());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            position: reader.u_block_pos(),
            new_block_runtime_id: reader.var_u32(),
            flags: reader.var_u32(),
            layer: reader.var_u32(),
            entity_unique_id: reader.var_i64(),
            transition_type: UpdateBlockTransition::from_u64(reader.var_u64()).unwrap(),
        }
    }
}
