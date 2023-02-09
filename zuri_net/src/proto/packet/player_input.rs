use glam::Vec2;

use zuri_net_derive::proto;

/// Sent by the client to the server when the player is moving but the server does not allow it to
/// update its movement using the MovePlayer packet. It includes situations where the player is
/// riding an entity like a boat. If this is the case, the packet is sent roughly every tick.
#[proto]
#[derive(Debug, Clone)]
pub struct PlayerInput {
    /// The movement vector of the input. It should be thought of in Pocket Edition controls, where
    /// specific the arrows (or a combination of two, resulting in a diagonal arrow) decide the
    /// direction of movement. The movement vector typically has a length of just one: Either it has
    /// movement on one axis, or it has a combination, resulting in `sqrt(2) / 2` for both axes.
    pub movement: Vec2,
    /// Indicates if the player was pressing the jump button during the input. It does not define if
    /// the player was actually in the air or not.
    pub jumping: bool,
    /// Indicates if the player was sneaking during the input. Note that this may also be checked by
    /// keeping the sneaking state updated using the PlayerAction packet.
    pub sneaking: bool,
}
