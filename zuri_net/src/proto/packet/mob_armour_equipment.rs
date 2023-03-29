use crate::proto::ints::VarU64;
use crate::proto::types::item::ItemInstance;
use zuri_net_derive::proto;

/// Sent by the server to the client to update the armour an entity is wearing. It is sent for both
/// players and other entities, such as zombies.
#[proto]
#[derive(Debug, Clone)]
pub struct MobArmourEquipment {
    /// The runtime ID of the entity. The runtime ID is unique for each world session, and entities
    /// are generally identified in packets using this runtime ID.
    pub entity_runtime_id: VarU64,
    /// The equipped helmet of the entity. Items that are not wearable on the head will not be
    /// rendered by the client. Unlike in Java Edition, blocks cannot be worn.
    pub helmet: ItemInstance,
    /// Chestplate is the chestplate of the entity. Items that are not wearable as chestplate will
    /// not be rendered.
    pub chestplate: ItemInstance,
    /// Leggings are the leggings of the entity. Items that are not wearable as leggings will not be
    /// rendered.
    pub leggings: ItemInstance,
    /// Boots are the boots of the entity. Items that are not wearable as boots will not be
    /// rendered.
    pub boots: ItemInstance,
}
