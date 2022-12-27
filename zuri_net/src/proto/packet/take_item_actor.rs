use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent by the server when a player picks up an item entity. It makes the item entity disappear to viewers and shows
/// the pick-up animation. The item entity is not actually removed from the world, but it is hidden from viewers.
#[derive(Debug)]
pub struct TakeItemActor {
    /// The entity runtime ID of the item that is being taken by another entity. It will disappear to viewers after
    /// showing the pick-up animation.
    pub item_entity_runtime_id: u64,
    /// The runtime ID of the entity that took the item, which is usually a player, but could be another entity like a
    /// zombie too.
    pub taker_entity_runtime_id: u64,
}

impl PacketType for TakeItemActor {
    fn write(&self, writer: &mut Writer) {
        writer.var_u64(self.item_entity_runtime_id);
        writer.var_u64(self.taker_entity_runtime_id);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            item_entity_runtime_id: reader.var_u64(),
            taker_entity_runtime_id: reader.var_u64(),
        }
    }
}
