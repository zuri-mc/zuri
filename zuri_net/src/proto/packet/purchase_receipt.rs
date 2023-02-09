use zuri_net_derive::proto;

use crate::proto::ints::VarU32;

/// Sent by the client to the server to notify the server it purchased an item from the Marketplace
/// store that was offered by the server. The packet is only used for partnered servers.
#[proto]
#[derive(Debug, Clone)]
pub struct PurchaseReceipt {
    /// A list of receipts, or proofs of purchases, for the offers that have been purchased by the
    /// player. This is used for server-side verification of the purchase.
    #[len_type(VarU32)]
    pub receipts: Vec<String>,
}
