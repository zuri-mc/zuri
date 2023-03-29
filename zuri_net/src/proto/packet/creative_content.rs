use crate::proto::ints::VarU32;
use crate::proto::types::item::ItemStack;
use zuri_net_derive::proto;

/// Sent by the server to set the creative inventory's content for a player. Introduced in 1.16,
/// this packet replaces the previous method - sending an InventoryContent packet with creative
/// inventory window ID. As of v1.16.100, this packet must be sent during the login sequence. Not
/// sending it will stop the client from joining the server.
#[proto(VarU32)]
#[derive(Debug, Clone)]
pub struct CreativeContent {
    /// A list of the items that should be added to the creative inventory.
    #[len_type(VarU32)]
    pub items: Vec<CreativeItem>,
}

/// A creative item present in the creative inventory.
#[proto]
#[derive(Debug, Clone)]
pub struct CreativeItem {
    /// A unique ID for the creative item. It has to be unique for each creative item sent to the
    /// client. An incrementing ID per creative item does the job.
    pub creative_item_network_id: VarU32,
    /// The item that should be added to the creative inventory.
    pub item: ItemStack,
}
