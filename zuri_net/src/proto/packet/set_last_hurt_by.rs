use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

#[derive(Debug, Clone)]
pub struct SetLastHurtBy {
    pub entity_type: i32,
}

impl PacketType for SetLastHurtBy {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.entity_type);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_type: reader.var_i32(),
        }
    }
}
