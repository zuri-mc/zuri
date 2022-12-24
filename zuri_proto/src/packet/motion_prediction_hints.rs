use glam::Vec3;

use crate::io::{Reader, Writer};
use crate::packet::PacketType;

#[derive(Debug)]
pub struct MotionPredictionHints {
    pub entity_runtime_id: u64,
    pub velocity: Vec3,
    pub on_ground: bool,
}

impl PacketType for MotionPredictionHints {
    fn write(&self, writer: &mut Writer) {
        writer.var_u64(self.entity_runtime_id);
        writer.vec3(self.velocity);
        writer.bool(self.on_ground);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_runtime_id: reader.var_u64(),
            velocity: reader.vec3(),
            on_ground: reader.bool(),
        }
    }
}
