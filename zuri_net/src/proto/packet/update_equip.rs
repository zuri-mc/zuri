use bytes::Bytes;
use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;
use crate::proto::types::inventory::Window;

/// Sent by the server to the client upon opening a horse inventory. It is used to set the content
/// of the inventory and specify additional properties, such as the items that are allowed to be put
/// in slots of the inventory.
#[derive(Debug, Clone)]
pub struct UpdateEquip {
    /// The window that the UpdateEquip packet concerns. It is the window sent for the horse
    /// inventory that was opened before this packet was sent.
    pub window: Window,
    /// The type of the window that was opened. Generally, this is the type of a horse inventory, as
    /// the packet is specifically made for that.
    pub window_type: u8,
    /// The size of the horse inventory that should be opened. A bigger size does, in fact, change
    /// the amount of slots displayed.
    pub size: i32,
    /// The unique ID of the entity whose equipment was 'updated' to the player. It is typically the
    /// horse entity that had its inventory opened.
    pub entity_unique_id: i64,
    /// Network NBT serialised compound holding the content of the inventory of the entity (the
    /// equipment) and additional data such as the allowed items for a particular slot, used to make
    /// sure only saddles can be put in the saddle slot etc.
    pub serialised_inventory_data: Bytes,
}

impl PacketType for UpdateEquip {
    fn write(&self, writer: &mut Writer) {
        writer.u8(self.window.to_u8().unwrap());
        writer.u8(self.window_type);
        writer.var_i32(self.size);
        writer.var_i64(self.entity_unique_id);
        writer.bytes(&self.serialised_inventory_data);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            window: Window::from_u8(reader.u8()).unwrap(),
            window_type: reader.u8(),
            size: reader.var_i32(),
            entity_unique_id: reader.var_i64(),
            serialised_inventory_data: reader.bytes(),
        }
    }
}
