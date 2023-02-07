use zuri_net_derive::proto;

use crate::proto::ints::VarU64;

/// Sent by the server when a player picks up an item entity. It makes the item entity disappear to
/// viewers and shows the pick-up animation. The item entity is not actually removed from the world,
/// but it is hidden from viewers.
#[proto]
#[derive(Debug, Clone)]
pub struct TakeItemActor {
    /// The entity runtime ID of the item that is being taken by another entity. It will disappear
    /// to viewers after showing the pick-up animation.
    pub item_entity_runtime_id: VarU64,
    /// The runtime ID of the entity that took the item, which is usually a player, but could be
    /// another entity like a zombie too.
    pub taker_entity_runtime_id: VarU64,
}
