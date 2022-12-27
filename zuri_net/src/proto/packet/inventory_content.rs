use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::packet::PacketType;
use crate::proto::io::{Reader, Writer};
use crate::proto::types::inventory::Window;
use crate::proto::types::item::ItemInstance;

/// Sent by the server to update the full content of a particular inventory. It is usually sent for the main inventory
/// of the player, but also works for other inventories that are currently opened by the player.
#[derive(Debug, Clone)]
pub struct InventoryContent {
    /// One of the windows that the client currently has opened, or a consistent one such as the main inventory.
    pub window: Window,
    /// The new content of the inventory. The length of this slice must be equal to the full size of the inventory
    /// window that was updated.
    pub content: Vec<ItemInstance>,
}

impl PacketType for InventoryContent {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.window.to_u32().unwrap());

        writer.var_u32(self.content.len() as u32);
        self.content.iter().for_each(|item| item.write(writer));
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            window: Window::from_u32(reader.var_u32()).unwrap(),
            content: (0..reader.var_u32()).map(|_| ItemInstance::read(reader)).collect(),
        }
    }
}
