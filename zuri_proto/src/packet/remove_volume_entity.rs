use crate::io::{Reader, Writer};
use crate::packet::Packet;

#[derive(Debug)]
pub struct RemoveVolumeEntity {
    pub entity_runtime_id: u64,
    pub dimension: Dimension,
}

impl Packet for RemoveVolumeEntity {
    fn write(&self, writer: &mut Writer) {
        writer.u64(self.entity_runtime_id);
        writer.var_i32(num::ToPrimitive::to_i32(&self.dimension).unwrap());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_runtime_id: reader.u64(),
            dimension: num::FromPrimitive::from_i32(reader.var_i32()).unwrap(),
        }
    }
}
