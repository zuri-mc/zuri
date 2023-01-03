use glam::Vec3;
use std::ops::{Div, Mul};

use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent by the server to play a sound to the client. Some of the sounds may only be started using
/// this packet and must be stopped using the StopSound packet.
#[derive(Debug, Clone)]
pub struct PlaySound {
    /// The name of the sound to play.
    pub sound_name: String,
    /// The position at which the sound was played. Some sounds do not depend on a position, which
    /// will then ignore it, but most of them will play with the direction based on the position
    /// compared to the player's position.
    pub position: Vec3,
    /// The relative volume of the sound to play. It will be less loud for the player if it is
    /// farther away from the position of the sound.
    pub volume: f32,
    /// The pitch of the sound to play. Some sounds completely ignore this field, whereas others use
    /// it to specify the pitch as the field is intended.
    pub pitch: f32,
}

impl PacketType for PlaySound {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.sound_name.as_str());
        writer.block_pos(self.position.mul(8.).as_ivec3());
        writer.f32(self.volume);
        writer.f32(self.pitch);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            sound_name: reader.string(),
            position: reader.block_pos().as_vec3().div(8.),
            volume: reader.f32(),
            pitch: reader.f32(),
        }
    }
}
