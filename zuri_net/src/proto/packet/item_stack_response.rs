use crate::proto::packet::PacketType;
use crate::proto::io::{Reader, Writer};
use crate::proto::types::item_stack::ItemStackResponseEntry;

#[derive(Debug, Clone)]
pub struct ItemStackResponse {
    pub responses: Vec<ItemStackResponseEntry>,
}

impl PacketType for ItemStackResponse {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.responses.len() as u32);
        self.responses.iter().for_each(|e| e.write(writer));
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            responses: (0..reader.var_u32())
                .map(|_| ItemStackResponseEntry::read(reader))
                .collect()
        }
    }
}
