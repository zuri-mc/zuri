use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::packet::PacketType;
use crate::proto::io::{Reader, Writer};
use crate::proto::types::inventory::Window;

/// Sent by the server to the client. It used to be used to link hot bar slots of the player to actual slots in the
/// inventory, but as of 1.2, this was changed and hot bar slots are no longer a free floating part of the inventory.
/// Since 1.2, the packet has been re-purposed, but its new functionality is not clear.
#[derive(Debug, Clone)]
pub struct PlayerHotBar {
    /// Before 1.2, this was the hot bar slot that is being linked to the inventory slot.
    pub selected_hotbar_slot: u32,
    /// The window that the hot bar slot is in.
    pub window: Window,
    /// The exact purpose of this field is unknown.
    pub select_hotbar_slot: bool,
}

impl PacketType for PlayerHotBar {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.selected_hotbar_slot);
        writer.u8(self.window.to_u8().unwrap());
        writer.bool(self.select_hotbar_slot);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            selected_hotbar_slot: reader.var_u32(),
            window: Window::from_u8(reader.u8()).unwrap(),
            select_hotbar_slot: reader.bool(),
        }
    }
}
