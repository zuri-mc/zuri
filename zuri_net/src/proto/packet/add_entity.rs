use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

#[derive(Debug, Clone)]
pub struct AddEntity {
    pub entity_network_id: u64,
}

impl PacketType for AddEntity {
    fn write(&self, writer: &mut Writer) {
        writer.var_u64(self.entity_network_id);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_network_id: reader.var_u64(),
        }
    }
}
