use crate::proto::ints::VarU32;
use glam::Vec3;
use num_derive::{FromPrimitive, ToPrimitive};
use zuri_net_derive::proto;

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
#[proto]
#[derive(Debug, Clone)]
pub struct UpdateClientInputLocks {
    /// An encoded bitset of all locks that are currently active.
    pub locks: VarU32,
    /// The server's position of the client at the time the packet was sent. It is unclear what the
    /// exact purpose of this field is.
    pub position: Vec3,
}
