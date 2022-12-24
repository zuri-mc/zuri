use crate::io::{Reader, Writer};
use crate::packet::Packet;

#[derive(Debug)]
pub struct SetLastHurtBy {
    pub entity_type: i32,
}

impl Packet for SetLastHurtBy {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.entity_type);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_type: reader.var_i32(),
        }
    }
}
