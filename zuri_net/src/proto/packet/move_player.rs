use crate::proto::ints::VarU64;
use glam::Vec3;
use zuri_net_derive::proto;

/// Sent by players to send their movement to the server, and by the server to update the movement
/// of player entities to other players. When using the new movement system, this is only sent by
/// the server.
#[proto]
#[derive(Debug, Clone)]
pub struct MovePlayer {
    /// The runtime ID of the player. The runtime ID is unique for each world session, and entities
    /// are generally identified in packets using this runtime ID.
    pub entity_runtime_id: VarU64,
    /// The position to spawn the player on. If the player is on a distance that the viewer cannot
    /// see it, the player will still show up if the viewer moves closer.
    pub position: Vec3,
    /// The vertical rotation of the player. Facing straight forward yields a pitch of zero. Pitch
    /// is measured in degrees.
    pub pitch: f32,
    /// The horizontal rotation of the player. Yaw is also measured in degrees.
    pub yaw: f32,
    /// The same as yaw, except that it applies specifically to the head of the player. A different
    /// value for head yaw than yaw means that the player will have its head turned.
    pub head_yaw: f32,
    /// The mode of the movement. It specifies the way the player's movement should be shown to
    /// other players.
    pub mode: MoveMode,
    /// The server tick at which the packet was sent. It is used in relation to
    /// CorrectPlayerMovePrediction.
    pub tick: VarU64,
}

#[proto(u8)]
#[derive(Debug, Clone, PartialEq)]
pub enum MoveMode {
    Normal(MoveModeNotTeleport),
    Reset(MoveModeNotTeleport),
    Teleport(MoveModeTeleport),
    Rotation(MoveModeNotTeleport),
}

#[proto(i32)]
#[derive(Debug, Clone, PartialEq)]
pub enum TeleportCause {
    None,
    Projectile,
    ChorusFruit,
    Command,
    Behaviour,
}

#[proto]
#[derive(Debug, Clone, PartialEq)]
pub struct MoveModeNotTeleport {
    /// Specifies if the player is considered on the ground. Note that proxies or hacked clients
    /// could fake this to always be true, so it should not be taken for granted.
    pub on_ground: bool,
    /// The runtime ID of the entity that the player might currently be riding. If not riding, this
    /// should be left zero.
    pub ridden_entity_runtime_id: VarU64,
}

#[proto]
#[derive(Debug, Clone, PartialEq)]
pub struct MoveModeTeleport {
    /// Specifies if the player is considered on the ground. Note that proxies or hacked clients
    /// could fake this to always be true, so it should not be taken for granted.
    pub on_ground: bool,
    /// The runtime ID of the entity that the player might currently be riding. If not riding, this
    /// should be left zero.
    pub ridden_entity_runtime_id: VarU64,
    /// Specifies the cause of the teleportation.
    pub teleport_cause: TeleportCause,
    /// The entity type that caused the teleportation, for example, an ender pearl.
    pub teleport_source_entity_type: i32,
}
