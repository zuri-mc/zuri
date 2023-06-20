use crate::proto::ints::VarU64;
use glam::Vec3;
use zuri_net_derive::proto;

/// Sent by the server to make a player respawn client-side. It is sent in response to a
/// PlayerAction packet with the action type Respawn. As of 1.13, the server sends two of these
/// packets with different states, and the client sends one of these back in order to complete the
/// respawn.
#[proto]
#[derive(Debug, Clone)]
pub struct Respawn {
    /// The position on which the player should be respawned. The position might be in a different
    /// dimension, in which case the client should first be sent a ChangeDimension packet.
    pub position: Vec3,
    /// The 'state' of the respawn. The value the packet contains depends on whether the server or
    /// client sends it.
    pub state: RespawnState,
    /// The entity runtime ID of the player that the respawn packet concerns. This is apparently for
    /// the server to recognise which player sends this packet.
    pub entity_runtime_id: VarU64,
}

#[proto(u8)]
#[derive(Debug, Clone)]
pub enum RespawnState {
    SearchingForSpawn,
    ReadyToSpawn,
    ClientReadyToSpawn,
}
