use glam::IVec3;
use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::packet::PacketType;
use crate::proto::io::{Reader, Writer};
use crate::proto::types::player::PlayerActionType;

/// Sent by the client when it executes any action, for example starting to sprint, swim, starting
/// the breaking of a block, dropping an item, etc.
#[derive(Debug, Clone)]
pub struct PlayerAction {
    /// The runtime ID of the player. The runtime ID is unique for each world session, and entities
    /// are generally identified in packets using this runtime ID.
    pub entity_runtime_id: u64,
    /// The type of action that was executed by the player.
    pub action_type: PlayerActionType,
    /// The position of the target block, if the action with the ActionType set concerned a block.
    /// If that is not the case, the block position will be zero.
    pub block_position: IVec3,
    /// The position of the action's result. When a UseItemOn action is sent, this is the position
    /// of the block clicked, but when a block is placed, this is the position at which the block
    /// will be placed.
    pub result_position: IVec3,
    /// The face of the target block that was touched. If the action with the ActionType set
    /// concerned a block. If not, the face is always zero.
    pub block_face: i32,
}

impl PacketType for PlayerAction {
    fn write(&self, writer: &mut Writer) {
        writer.var_u64(self.entity_runtime_id);
        writer.var_i32(self.action_type.to_i32().unwrap());
        writer.u_block_pos(self.block_position);
        writer.u_block_pos(self.result_position);
        writer.var_i32(self.block_face);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_runtime_id: reader.var_u64(),
            action_type: PlayerActionType::from_i32(reader.var_i32()).unwrap(),
            block_position: reader.u_block_pos(),
            result_position: reader.u_block_pos(),
            block_face: reader.var_i32(),
        }
    }
}
