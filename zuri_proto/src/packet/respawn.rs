use glam::Vec3;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

use crate::packet::Packet;
use crate::io::{Reader, Writer};

/// Sent by the server to make a player respawn client-side. It is sent in response to a PlayerAction packet with the
/// action type Respawn. As of 1.13, the server sends two of these packets with different states, and the client sends
/// one of these back in order to complete the respawn.
#[derive(Debug)]
pub struct Respawn {
    /// The position on which the player should be respawned. The position might be in a different dimension, in which
    /// case the client should first be sent a ChangeDimension packet.
    pub position: Vec3,
    /// The 'state' of the respawn. It is one of the constants that may be found above, and the value the packet
    /// contains depends on whether the server or client sends it.
    pub state: RespawnState,
    /// The entity runtime ID of the player that the respawn packet concerns. This is apparently for the server to
    /// recognise which player sends this packet.
    pub entity_runtime_id: u64,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum RespawnState {
    SearchingForSpawn,
    ReadyToSpawn,
    ClientReadyToSpawn,
}

impl Packet for Respawn {
    fn write(&self, writer: &mut Writer) {
        writer.vec3(self.position);
        writer.u8(self.state.to_u8().unwrap());
        writer.var_u64(self.entity_runtime_id);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            position: reader.vec3(),
            state: RespawnState::from_u8(reader.u8()).unwrap(),
            entity_runtime_id: reader.var_u64(),
        }
    }
}
