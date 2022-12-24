use crate::io::{Reader, Writer};
use crate::packet::Packet;

#[derive(Debug)]
pub struct RemoveEntity {
    pub entity_network_id: u64,
}

impl Packet for RemoveEntity {
    fn write(&self, writer: &mut Writer) {
        writer.var_u64(self.entity_network_id);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_network_id: reader.var_u64(),
        }
    }
}
