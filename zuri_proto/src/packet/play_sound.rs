use glam::Vec3;
use std::ops::{Div, Mul};

use crate::io::{Reader, Writer};
use crate::packet::Packet;

#[derive(Debug)]
pub struct PlaySound {
    pub sound_name: String,
    pub position: Vec3,
    pub volume: f32,
    pub pitch: f32,
}

impl Packet for PlaySound {
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
