use zuri_net_derive::proto;

use crate::proto::ints::VarU32;
use crate::proto::types::item_stack::ItemStackRequestEntry;

/// Sent by the client to change item stacks in an inventory. It is essentially a replacement of the
/// InventoryTransaction packet added in 1.16 for inventory specific actions, such as moving items
/// around or crafting. The InventoryTransaction packet is still used for actions such as placing
/// blocks and interacting with entities.
#[proto]
#[derive(Debug, Clone)]
pub struct ItemStackRequest {
    /// A list of item stack requests. These requests are all separate, but the client buffers the
    /// requests, so you might find multiple unrelated requests in this packet.
    #[len_type(VarU32)]
    pub requests: Vec<ItemStackRequestEntry>,
}
