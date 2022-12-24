use num_traits::{FromPrimitive, ToPrimitive};

use crate::packet::Packet;
use crate::io::{Reader, Writer};
use crate::types::inventory::Window;
use crate::types::item::ItemInstance;

/// Sent by the server to update a single slot in one of the inventory windows that the client currently has opened.
/// Usually this is the main inventory, but it may also be the off hand or, for example, a chest inventory.
#[derive(Debug)]
pub struct InventorySlot {
    /// The window that the packet modifies. It must point to one of the windows that the client currently has opened.
    pub window: Window,
    /// The index of the slot that the packet modifies. The new item will be set to the slot at this index.
    pub slot: u32,
    /// The item to be put in the slot. It will overwrite any item that may currently be present in that slot.
    pub new_item: ItemInstance,
}

impl Packet for InventorySlot {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.window.to_u32().unwrap());
        writer.var_u32(self.slot);
        self.new_item.write(writer);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            window: Window::from_u32(reader.var_u32()).unwrap(),
            slot: reader.var_u32(),
            new_item: ItemInstance::read(reader),
        }
    }
}
