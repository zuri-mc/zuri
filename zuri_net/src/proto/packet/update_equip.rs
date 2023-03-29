use crate::proto::ints::{VarI32, VarI64};
use bytes::Bytes;
use zuri_net_derive::proto;

use crate::proto::types::inventory::Window;

/// Sent by the server to the client upon opening a horse inventory. It is used to set the content
/// of the inventory and specify additional properties, such as the items that are allowed to be put
/// in slots of the inventory.
#[proto]
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
    pub size: VarI32,
    /// The unique ID of the entity whose equipment was 'updated' to the player. It is typically the
    /// horse entity that had its inventory opened.
    pub entity_unique_id: VarI64,
    /// Network NBT serialised compound holding the content of the inventory of the entity (the
    /// equipment) and additional data such as the allowed items for a particular slot, used to make
    /// sure only saddles can be put in the saddle slot etc.
    pub serialised_inventory_data: Bytes,
}
