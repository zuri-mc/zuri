use glam::Vec3;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{ToPrimitive, FromPrimitive};
use crate::io::{Reader, Writer};
use crate::packet::Packet;

#[derive(Debug)]
pub struct UpdateClientInputLocks {
    pub locks: ClientInputLock,
    pub position: Vec3,
}


#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum ClientInputLock {
    Move,
    Jump,
    Sneak,
    Mount,
    Dismount,
    Rotation,
}

impl Packet for UpdateClientInputLocks {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.locks.to_u32().unwrap());
        writer.vec3(self.position);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            locks: ClientInputLock::from_u32(reader.var_u32()).unwrap(),
            position: reader.vec3(),
        }
    }
}
