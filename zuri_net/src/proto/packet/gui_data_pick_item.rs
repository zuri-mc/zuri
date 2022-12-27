use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

#[derive(Debug)]
pub struct GUIDataPickItem {
    pub item_name: String,
    pub item_effects: String,
    pub hot_bar_slot: i32,
}

impl PacketType for GUIDataPickItem {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.item_name.as_str());
        writer.string(self.item_effects.as_str());
        writer.i32(self.hot_bar_slot);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            item_name: reader.string(),
            item_effects: reader.string(),
            hot_bar_slot: reader.i32(),
        }
    }
}
