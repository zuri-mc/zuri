use crate::proto::packet::PacketType;
use crate::proto::io::{Reader, Writer};
use crate::proto::types::item::ItemInstance;

/// Sent by the server to the client to update the armour an entity is wearing. It is sent for both
/// players and other entities, such as zombies.
#[derive(Debug, Clone)]
pub struct MobArmourEquipment {
    /// The runtime ID of the entity. The runtime ID is unique for each world session, and entities
    /// are generally identified in packets using this runtime ID.
    pub entity_runtime_id: u64,
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

impl PacketType for MobArmourEquipment {
    fn write(&self, writer: &mut Writer) {
        writer.var_u64(self.entity_runtime_id);

        self.helmet.write(writer);
        self.chestplate.write(writer);
        self.leggings.write(writer);
        self.boots.write(writer);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_runtime_id: reader.var_u64(),

            helmet: ItemInstance::read(reader),
            chestplate: ItemInstance::read(reader),
            leggings: ItemInstance::read(reader),
            boots: ItemInstance::read(reader),
        }
    }
}
