use glam::IVec3;
use crate::io::{Reader, Writer};
use crate::packet::Packet;
use crate::types::world::{Dimension, SubChunkOffset};

#[derive(Debug)]
pub struct SubChunkRequest {
    pub dimension: Dimension,
    pub position: IVec3,
    pub offsets: Vec<SubChunkOffset>,
}

impl Packet for SubChunkRequest {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(num::ToPrimitive::to_i32(&self.dimension).unwrap());
        writer.block_pos(self.position);

        writer.u32(self.offsets.len() as u32);
        self.offsets.iter().for_each(|offset| offset.write(writer));
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            dimension: num::FromPrimitive::from_i32(reader.var_i32()).unwrap(),
            position: reader.block_pos(),
            offsets: (0..reader.u32()).map(|_| SubChunkOffset::read(reader)).collect(),
        }
    }
}
