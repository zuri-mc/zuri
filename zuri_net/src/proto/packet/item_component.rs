use zuri_net_derive::proto;

use crate::proto::ints::VarU32;
use crate::proto::types::item_stack::ItemComponentEntry;

/// Sent by the server to attach client-side components to a custom item.
#[proto]
#[derive(Debug, Clone)]
pub struct ItemComponent {
    /// A list of all custom items with their respective components set.
    #[len_type(VarU32)]
    pub items: Vec<ItemComponentEntry>,
}
