use zuri_net_derive::packet;

use crate::proto::ints::VarU32;
use crate::proto::packet::PacketType;
use crate::proto::types::item_stack::ItemComponentEntry;

/// Sent by the server to attach client-side components to a custom item.
#[packet]
#[derive(Debug, Clone)]
pub struct ItemComponent {
    /// A list of all custom items with their respective components set.
    #[size_type(VarU32)]
    pub items: Vec<ItemComponentEntry>,
}
