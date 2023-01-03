use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent by the server to make the client 'select' a hot bar slot. It currently appears to be broken
/// however, and does not actually set the selected slot to the hot bar slot set in the packet.
#[derive(Debug, Clone)]
pub struct GUIDataPickItem {
    /// The name of the item that shows up in the top part of the popup that shows up when selecting
    /// an item. It is shown as if an item was selected by the player itself.
    pub item_name: String,
    /// The line under the ItemName, where the effects of the item are usually situated.
    pub item_effects: String,
    /// The hot bar slot to be selected/picked. This does not currently work, so it does not matter
    /// what number this is.
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
