use crate::proto::ints::VarU32;
use zuri_net_derive::proto;

use crate::proto::types::inventory::Window;
use crate::proto::types::item::ItemInstance;

/// Sent by the server to update the full content of a particular inventory. It is usually sent for
/// the main inventory of the player, but also works for other inventories that are currently opened
/// by the player.
#[proto]
#[derive(Debug, Clone)]
pub struct InventoryContent {
    /// One of the windows that the client currently has opened, or a consistent one such as the
    /// main inventory.
    #[enum_header(VarU32)]
    pub window: Window,
    /// The new content of the inventory. The length of this list must be equal to the full size of
    /// the inventory window that was updated.
    #[len_type(VarU32)]
    pub content: Vec<ItemInstance>,
}
