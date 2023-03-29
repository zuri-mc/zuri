use crate::proto::ints::VarI32;
use zuri_net_derive::proto;

use crate::proto::io::UBlockPos;
use crate::proto::types::world::{Dimension, SpawnType};

/// Sent by the server to update the spawn position of a player, for example when sleeping in a bed.
#[proto]
#[derive(Debug, Clone)]
pub struct SetSpawnPosition {
    /// Specifies the behaviour of the spawn set. If World is set, the position that compasses will
    /// point to is changed.
    pub spawn_type: SpawnType,
    /// The new position of the spawn that was set. If the spawn type is World, compasses will point
    /// to this position. As of 1.16, position is always the position of the player.
    pub position: UBlockPos,
    /// The dimension that had its spawn updated. This is specifically relevant for behaviour added
    /// in 1.16 such as the respawn anchor, which allows setting the spawn in a specific dimension.
    #[enum_header(VarI32)]
    pub dimension: Dimension,
    /// A new field added in 1.16. It holds the spawn position of the world. This spawn position is
    /// `{-i32::MIN, -i32::MIN, -i32::MIN}` for a default spawn position.
    pub spawn_position: UBlockPos,
}
