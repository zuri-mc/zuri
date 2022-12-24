use glam::IVec3;
use num_traits::{FromPrimitive, ToPrimitive};

use crate::packet::PacketType;
use crate::io::{Reader, Writer};
use crate::types::world::{Dimension, SpawnType};

/// Sent by the server to update the spawn position of a player, for example when sleeping in a bed.
#[derive(Debug)]
pub struct SetSpawnPosition {
    /// Specifies the behaviour of the spawn set. If World is set, the position that compasses will point to is changed.
    pub spawn_type: SpawnType,
    /// The new position of the spawn that was set. If the spawn type is World, compasses will point to this position.
    /// As of 1.16, position is always the position of the player.
    pub position: IVec3,
    /// The dimension that had its spawn updated. This is specifically relevant for behaviour added in 1.16 such as the
    /// respawn anchor, which allows setting the spawn in a specific dimension.
    pub dimension: Dimension,
    /// A new field added in 1.16. It holds the spawn position of the world. This spawn position is
    /// {-i32::MIN, -i32::MIN, -i32::MIN} for a default spawn position.
    pub spawn_position: IVec3,
}

impl PacketType for SetSpawnPosition {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.spawn_type.to_i32().unwrap());
        writer.u_block_pos(self.position);
        writer.var_i32(self.dimension.to_i32().unwrap());
        writer.u_block_pos(self.spawn_position);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            spawn_type: SpawnType::from_i32(reader.var_i32()).unwrap(),
            position: reader.u_block_pos(),
            dimension: Dimension::from_i32(reader.var_i32()).unwrap(),
            spawn_position: reader.u_block_pos(),
        }
    }
}
