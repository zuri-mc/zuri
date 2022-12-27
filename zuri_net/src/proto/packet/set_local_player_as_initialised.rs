use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

#[derive(Debug)]
pub struct SetLocalPlayerAsInitialised {
    pub entity_runtime_id: u64,
}

impl PacketType for SetLocalPlayerAsInitialised {
    fn write(&self, writer: &mut Writer) {
        writer.var_u64(self.entity_runtime_id);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_runtime_id: reader.var_u64(),
        }
    }
}
