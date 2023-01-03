use glam::Vec3;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{ToPrimitive, FromPrimitive};
use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

#[derive(Debug, Copy, Clone, FromPrimitive, ToPrimitive)]
pub enum ClientInputLock {
    Move,
    Jump,
    Sneak,
    Mount,
    Dismount,
    Rotation,
}

impl ClientInputLock {
    pub fn flag(&self) -> u32 {
        1 << ((*self as u32) + 1)
    }
}

/// Sent by the server to the client to lock certain inputs the client usually has, such as
/// movement, jumping, sneaking, and more.
#[derive(Debug, Clone)]
pub struct UpdateClientInputLocks {
    /// An encoded bitset of all locks that are currently active.
    pub locks: u32,
    /// The server's position of the client at the time the packet was sent. It is unclear what the
    /// exact purpose of this field is.
    pub position: Vec3,
}

impl PacketType for UpdateClientInputLocks {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.locks);
        writer.vec3(self.position);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            locks: reader.var_u32(),
            position: reader.vec3(),
        }
    }
}
