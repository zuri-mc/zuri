use zuri_net_derive::packet;

/// Sent by the client when it tries to pick an entity, so that it gets a spawn egg which can spawn
/// that entity.
#[packet]
#[derive(Debug, Clone)]
pub struct ActorPickRequest {
    /// The unique ID of the entity that was attempted to be picked. The server must find the type
    /// of that entity and provide the correct spawn egg to the player.
    pub entity_unique_id: i64,
    /// The held hot bar slot of the player at the time of trying to pick the entity. If empty, the
    /// resulting spawn egg should be put into this slot.
    pub hotbar_slot: u8,
    /// True if the pick request requests the entity metadata.
    pub with_data: bool,
}
