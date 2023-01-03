use glam::Vec3;
use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;
use crate::proto::types::world::Dimension;

/// Sent by the server to the client to send a dimension change screen client-side. Once the screen
/// is cleared client-side, the client will send a PlayerAction packet with the dimension change
/// done action attached.
#[derive(Debug, Clone)]
pub struct ChangeDimension {
    /// The dimension that the client should be changed to. The dimension must be different from the
    /// one the player is currently in, otherwise the client will freeze on the screen.
    pub dimension: Dimension,
    /// The position in the new dimension that the player is spawned in.
    pub position: Vec3,
    /// Specifies if the dimension change was respawn based, meaning that the player died in one
    /// dimension and got respawned into another. The client will send a PlayerAction packet with
    /// dimension change request attached as the action if it dies in another dimension, indicating
    /// that it needs a DimensionChange packet with respawn set to true.
    pub respawn: bool,
}

impl PacketType for ChangeDimension {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.dimension.to_i32().unwrap());
        writer.vec3(self.position);
        writer.bool(self.respawn);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            dimension: Dimension::from_i32(reader.var_i32()).unwrap(),
            position: reader.vec3(),
            respawn: reader.bool(),
        }
    }
}
