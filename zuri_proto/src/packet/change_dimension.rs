use glam::Vec3;
use num_traits::{FromPrimitive, ToPrimitive};

use crate::io::{Reader, Writer};
use crate::packet::Packet;
use crate::types::world::Dimension;

#[derive(Debug)]
pub struct ChangeDimension {
    pub dimension: Dimension,
    pub position: Vec3,
    pub respawn: bool,
}

impl Packet for ChangeDimension {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.dimension.to_i32().unwrap());
        writer.vec3(self.position);
        writer.bool(self.respawn);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            dimension: Dimension::from_i32(reader.var_i32()).unwrap(),
            position: reader.vec3(),
            respawn: reader.bool(),
        }
    }
}
