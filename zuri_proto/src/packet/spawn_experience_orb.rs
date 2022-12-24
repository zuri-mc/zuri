use glam::Vec3;
use crate::io::{Reader, Writer};
use crate::packet::PacketType;

#[derive(Debug)]
pub struct SpawnExperienceOrb {
    pub position: Vec3,
    pub experience_amount: i32,
}

impl PacketType for SpawnExperienceOrb {
    fn write(&self, writer: &mut Writer) {
        writer.vec3(self.position);
        writer.var_i32(self.experience_amount);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            position: reader.vec3(),
            experience_amount: reader.var_i32(),
        }
    }
}
