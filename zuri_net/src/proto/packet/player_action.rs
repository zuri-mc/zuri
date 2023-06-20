use crate::proto::ints::{VarI32, VarU64};
use zuri_net_derive::proto;

use crate::proto::io::UBlockPos;
use crate::proto::types::player::PlayerActionType;

/// Sent by the client when it executes any action, for example starting to sprint, swim, starting
/// the breaking of a block, dropping an item, etc.
#[proto]
#[derive(Debug, Clone)]
pub struct PlayerAction {
    /// The runtime ID of the player. The runtime ID is unique for each world session, and entities
    /// are generally identified in packets using this runtime ID.
    pub entity_runtime_id: VarU64,
    /// The type of action that was executed by the player.
    pub action_type: PlayerActionType,
    /// The position of the target block, if the action with the ActionType set concerned a block.
    /// If that is not the case, the block position will be zero.
    pub block_position: UBlockPos,
    /// The position of the action's result. When a UseItemOn action is sent, this is the position
    /// of the block clicked, but when a block is placed, this is the position at which the block
    /// will be placed.
    pub result_position: UBlockPos,
    /// The face of the target block that was touched. If the action with the ActionType set
    /// concerned a block. If not, the face is always zero.
    pub block_face: VarI32,
}
