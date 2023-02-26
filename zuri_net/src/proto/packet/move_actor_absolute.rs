use glam::Vec3;
use num_derive::{FromPrimitive, ToPrimitive};

use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum MoveFlag {
    OnGround,
    Teleport,
}

/// Sent by the server to move an entity to an absolute position. It is typically used for movements
/// where high accuracy isn't needed, such as for long range teleporting.
#[derive(Debug, Clone)]
pub struct MoveActorAbsolute {
    /// The runtime ID of the entity. The runtime ID is unique for each world session, and entities
    /// are generally identified in packets using this runtime ID.
    pub entity_runtime_id: u64,
    /// A combination of MoveFlags that specify details of the movement.
    pub flags: u8,
    /// The position to move the entity to. If the entity is on a distance that the player cannot
    /// see it, the entity will still show up if the player moves closer.
    pub position: Vec3,
    /// The rotation of the entity. The first value is the pitch, the second is the head yaw, and
    /// the third is the yaw.
    pub rotation: Vec3,
}

impl PacketType for MoveActorAbsolute {
    fn write(&self, writer: &mut Writer) {
        writer.var_u64(self.entity_runtime_id);

        writer.u8(self.flags);

        writer.vec3(self.position);
        writer.byte_f32(self.rotation.x);
        writer.byte_f32(self.rotation.y);
        writer.byte_f32(self.rotation.z);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_runtime_id: reader.var_u64(),

            flags: reader.u8(),

            position: reader.vec3(),
            rotation: Vec3 {
                x: reader.byte_f32(),
                y: reader.byte_f32(),
                z: reader.byte_f32(),
            },
        }
    }
}
