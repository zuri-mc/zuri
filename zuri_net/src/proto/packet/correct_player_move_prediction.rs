use glam::Vec3;

use zuri_net_derive::proto;

use crate::proto::ints::VarU64;

/// Sent by the server if and only if server authoritative movement with rewind is enabled. The
/// packet is used to correct movement at a specific point in time.
#[proto]
#[derive(Debug, Clone)]
pub struct CorrectPlayerMovePrediction {
    /// The position that the player is supposed to be at the tick written in the field below. The
    /// client will change its current position based on movement after that tick starting from the
    /// position.
    pub position: Vec3,
    /// The change in position compared to what the client sent at that specific tick.
    pub delta: Vec3,
    /// Specifies if the player was on the ground at the time of the tick below.
    pub on_ground: bool,
    /// The tick of the movement which was corrected by this packet.
    pub tick: VarU64,
}
