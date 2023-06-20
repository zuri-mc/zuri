use crate::proto::ints::VarU32;
use zuri_net_derive::proto;

use crate::proto::types::inventory::Window;
use crate::proto::types::item::ItemInstance;

/// Sent by the server to update a single slot in one of the inventory windows that the client
/// currently has opened. Usually this is the main inventory, but it may also be the off hand or,
/// for example, a chest inventory.
#[proto]
#[derive(Debug, Clone)]
pub struct InventorySlot {
    /// The window that the packet modifies. It must point to one of the windows that the client
    /// currently has opened.
    #[enum_header(VarU32)]
    pub window: Window,
    /// The index of the slot that the packet modifies. The new item will be set to the slot at this
    /// index.
    pub slot: VarU32,
    /// The item to be put in the slot. It will overwrite any item that may currently be present in
    /// that slot.
    pub new_item: ItemInstance,
}
