use glam::Vec3;
use crate::io::{Reader, Writer};
use crate::packet::Packet;

#[derive(Debug)]
pub struct CorrectPlayerMovePrediction {
    pub position: Vec3,
    pub delta: Vec3,
    pub on_ground: bool,
    pub tick: u64,
}

impl Packet for CorrectPlayerMovePrediction {
    fn write(&self, writer: &mut Writer) {
        writer.vec3(self.position);
        writer.vec3(self.delta);
        writer.bool(self.on_ground);
        writer.var_u64(self.tick);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            position: reader.vec3(),
            delta: reader.vec3(),
            on_ground: reader.bool(),
            tick: reader.var_u64(),
        }
    }
}
