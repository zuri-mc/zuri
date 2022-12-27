use glam::Vec3;
use num_derive::{FromPrimitive, ToPrimitive};
use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

#[derive(Debug)]
pub struct MoveActorDelta {
    pub entity_runtime_id: u64,
    pub flags: u16,
    pub position: Vec3,
    pub rotation: Vec3,
}

impl PacketType for MoveActorDelta {
    fn write(&self, writer: &mut Writer) {
        writer.var_u64(self.entity_runtime_id);
        writer.u16(self.flags);
        if self.flags & MoveActorDeltaFlag::HasX.flag() != 0 {
            writer.f32(self.position.x);
        }
        if self.flags & MoveActorDeltaFlag::HasY.flag() != 0 {
            writer.f32(self.position.y);
        }
        if self.flags & MoveActorDeltaFlag::HasZ.flag() != 0 {
            writer.f32(self.position.z);
        }
        if self.flags & MoveActorDeltaFlag::HasRotX.flag() != 0 {
            writer.byte_f32(self.rotation.x);
        }
        if self.flags & MoveActorDeltaFlag::HasRotY.flag() != 0 {
            writer.byte_f32(self.rotation.y);
        }
        if self.flags & MoveActorDeltaFlag::HasRotZ.flag() != 0 {
            writer.byte_f32(self.rotation.z);
        }
    }

    fn read(reader: &mut Reader) -> Self {
        let entity_runtime_id = reader.var_u64();
        let flags = reader.u16();
        Self {
            entity_runtime_id,
            flags,
            position: {
                let mut position = Vec3::default();
                if flags & MoveActorDeltaFlag::HasX.flag() != 0 {
                    position.x = reader.f32();
                }
                if flags & MoveActorDeltaFlag::HasY.flag() != 0 {
                    position.y = reader.f32();
                }
                if flags & MoveActorDeltaFlag::HasZ.flag() != 0 {
                    position.z = reader.f32();
                }
                position
            },
            rotation: {
                let mut rotation = Vec3::default();
                if flags & MoveActorDeltaFlag::HasRotX.flag() != 0 {
                    rotation.x = reader.byte_f32();
                }
                if flags & MoveActorDeltaFlag::HasRotY.flag() != 0 {
                    rotation.y = reader.byte_f32();
                }
                if flags & MoveActorDeltaFlag::HasRotZ.flag() != 0 {
                    rotation.z = reader.byte_f32();
                }
                rotation
            },
        }
    }
}

#[derive(Clone, Copy, Debug, FromPrimitive, ToPrimitive)]
pub enum MoveActorDeltaFlag {
    HasX,
    HasY,
    HasZ,
    HasRotX,
    HasRotY,
    HasRotZ,
    OnGround,
    Teleport,
    ForceMove,
}

impl MoveActorDeltaFlag {
    pub fn flag(&self) -> u16 {
        1 << (*self as u16)
    }
}
