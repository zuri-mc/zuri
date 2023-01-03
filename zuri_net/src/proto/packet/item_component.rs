use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;
use crate::proto::types::item_stack::ItemComponentEntry;

/// Sent by the server to attach client-side components to a custom item.
#[derive(Debug, Clone)]
pub struct ItemComponent {
    /// A list of all custom items with their respective components set.
    pub items: Vec<ItemComponentEntry>,
}

impl PacketType for ItemComponent {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.items.len() as u32);
        self.items.iter().for_each(|entry| entry.write(writer));
    }

    fn read(reader: &mut Reader) -> Self {
        Self { items: (0..reader.var_u32()).map(|_| ItemComponentEntry::read(reader)).collect() }
    }
}
