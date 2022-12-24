/// Sent by the client when it tries to pick an entity, so that it gets a spawn egg which can spawn that entity.
#[derive(Debug)]
pub struct ActorPickRequest {
    /// The unique ID of the entity that was attempted to be picked. The server must find the type of that entity and
    /// provide the correct spawn egg to the player.
    pub entity_unique_id: i64,
    /// The held hot bar slot of the player at the time of trying to pick the entity. If empty, the resulting spawn egg
    /// should be put into this slot.
    pub hotbar_slot: u8,
    /// True if the pick request requests the entity metadata.
    pub with_data: bool,
}

impl Packet for ActorPickRequest {
    fn write(&self, writer: &mut Writer) {
        writer.i64(self.entity_unique_id);
        writer.u8(self.hotbar_slot);
        writer.bool(self.with_data);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_unique_id: reader.i64(),
            hotbar_slot: reader.u8(),
            with_data: reader.bool(),
        }
    }
}
