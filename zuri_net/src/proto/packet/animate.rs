use crate::proto::ints::{VarI32, VarU64};
use zuri_net_derive::proto;

#[proto(VarI32)]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
/// Each variant contains the runtime ID of the player that the animation should be played upon. The
/// runtime ID is unique for each world session, and entities are generally identified in packets
/// using this runtime ID.
pub enum AnimateAction {
    SwingArm(VarU64) = 1,
    StopSleep(VarU64) = 3,
    CriticalHit(VarU64) = 4,
    MagicCriticalHit(VarU64) = 5,
    /// It is unclear what the second field, `boat_rowing_time`, is for.
    RowRight(VarU64, f32) = 128,
    /// It is unclear what the second field, `boat_rowing_time`, is for.
    RowLeft(VarU64, f32) = 129,
}

/// Sent by the server to send a player animation from one player to all viewers of that player. It
/// is used for a couple of actions, such as arm swimming and critical hits.\
#[proto]
#[derive(Debug, Clone)]
pub struct Animate {
    /// The action type to execute.
    pub action_type: AnimateAction,
}
