use glam::Vec3;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

#[derive(Debug, PartialEq, FromPrimitive, ToPrimitive)]
pub enum MoveMode {
    Normal,
    Reset,
    Teleport,
    Rotation,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum TeleportCause {
    None,
    Projectile,
    ChorusFruit,
    Command,
    Behaviour,
}

/// Sent by players to send their movement to the server, and by the server to update the movement of player entities
/// to other players. When using the new movement system, this is only sent by the server.
#[derive(Debug)]
pub struct MovePlayer {
    /// The runtime ID of the player. The runtime ID is unique for each world session, and entities are generally
    /// identified in packets using this runtime ID.
    pub entity_runtime_id: u64,
    /// The position to spawn the player on. If the player is on a distance that the viewer cannot see it, the player
    /// will still show up if the viewer moves closer.
    pub position: Vec3,
    /// The vertical rotation of the player. Facing straight forward yields a pitch of 0. Pitch is measured in degrees.
    pub pitch: f32,
    /// The horizontal rotation of the player. Yaw is also measured in degrees.
    pub yaw: f32,
    /// The same as yaw, except that it applies specifically to the head of the player. A different value for head_yaw
    /// than yaw means that the player will have its head turned.
    pub head_yaw: f32,
    /// The mode of the movement. It specifies the way the player's movement should be shown to other players.
    pub mode: MoveMode,
    /// Specifies if the player is considered on the ground. Note that proxies or hacked clients could fake this to
    /// always be true, so it should not be taken for granted.
    pub on_ground: bool,
    /// The runtime ID of the entity that the player might currently be riding. If not riding, this should be left zero.
    pub ridden_entity_runtime_id: u64,
    /// Written only if mode is Teleport. It specifies the cause of the teleportation.
    pub teleport_cause: TeleportCause,
    /// The entity type that caused the teleportation, for example, an ender pearl.
    pub teleport_source_entity_type: i32,
    /// The server tick at which the packet was sent. It is used in relation to CorrectPlayerMovePrediction.
    pub tick: u64,
}

impl PacketType for MovePlayer {
    fn write(&self, writer: &mut Writer) {
        writer.var_u64(self.entity_runtime_id);

        writer.vec3(self.position);
        writer.f32(self.pitch);
        writer.f32(self.yaw);
        writer.f32(self.head_yaw);

        writer.u8(self.mode.to_u8().unwrap());
        writer.bool(self.on_ground);
        writer.var_u64(self.ridden_entity_runtime_id);
        if self.mode == MoveMode::Teleport {
            writer.i32(self.teleport_cause.to_i32().unwrap());
            writer.i32(self.teleport_source_entity_type);
        }

        writer.var_u64(self.tick);
    }

    fn read(reader: &mut Reader) -> Self {
        let mut packet = Self {
            entity_runtime_id: reader.var_u64(),
            position: reader.vec3(),
            pitch: reader.f32(),
            yaw: reader.f32(),
            head_yaw: reader.f32(),
            mode: MoveMode::from_u8(reader.u8()).unwrap(),
            on_ground: reader.bool(),
            ridden_entity_runtime_id: reader.var_u64(),
            teleport_cause: TeleportCause::None,
            teleport_source_entity_type: 0,
            tick: 0,
        };
        if packet.mode == MoveMode::Teleport {
            packet.teleport_cause = TeleportCause::from_i32(reader.i32()).unwrap();
            packet.teleport_source_entity_type = reader.i32();
        }
        packet.tick = reader.var_u64();

        packet
    }
}
