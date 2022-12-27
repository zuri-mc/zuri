use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::packet::PacketType;
use crate::proto::io::{Reader, Writer};
use crate::proto::types::world::Dimension;

#[derive(Debug)]
pub struct RemoveVolumeEntity {
    pub entity_runtime_id: u64,
    pub dimension: Dimension,
}

impl PacketType for RemoveVolumeEntity {
    fn write(&self, writer: &mut Writer) {
        writer.u64(self.entity_runtime_id);
        writer.var_i32(self.dimension.to_i32().unwrap());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_runtime_id: reader.u64(),
            dimension: Dimension::from_i32(reader.var_i32()).unwrap(),
        }
    }
}
