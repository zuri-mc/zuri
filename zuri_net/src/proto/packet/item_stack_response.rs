use zuri_net_derive::proto;

use crate::proto::ints::VarU32;
use crate::proto::types::item_stack::ItemStackResponseEntry;

/// Sent by the server in response to an ItemStackRequest packet from the client. This packet is
/// used to either approve or reject ItemStackRequests from the client. If a request is approved,
/// the client will simply continue as normal. If rejected, the client will undo the actions so that
/// the inventory should be in sync with the server again.
#[proto]
#[derive(Debug, Clone)]
pub struct ItemStackResponse {
    /// A list of responses to ItemStackRequests sent by the client before. Responses either approve
    /// or reject a request from the client. Vanilla limits the size of this list to 4096.
    #[len_type(VarU32)]
    pub responses: Vec<ItemStackResponseEntry>,
}
