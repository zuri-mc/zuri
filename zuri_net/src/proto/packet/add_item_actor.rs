use std::collections::HashMap;

use glam::Vec3;

use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;
use crate::proto::types::entity_data::EntityDataEntry;
use crate::proto::types::item::ItemInstance;

/// Sent by the server to the client to make an item entity show up. It is one of the few entities
/// that cannot be sent using the AddActor packet
#[derive(Debug, Clone)]
pub struct AddItemActor {
    /// The unique ID of the entity. The unique ID is a value that remains consistent across
    /// different sessions of the same world, but most servers simply fill the runtime ID of the
    /// entity out for this field.
    pub entity_unique_id: i64,
    /// The runtime ID of the entity. The runtime ID is unique for each world session, and entities
    /// are generally identified in packets using this runtime ID.
    pub entity_runtime_id: u64,
    /// The item that is spawned. It must have a valid ID for it to show up client-side. If it is
    /// not a valid item, the client will crash when coming near.
    pub item: ItemInstance,
    /// The position to spawn the entity on. If the entity is on a distance that the player cannot
    /// see it, the entity will still show up if the player moves closer.
    pub position: Vec3,
    /// The initial velocity the entity spawns with. This velocity will initiate client side
    /// movement of the entity.
    pub velocity: Vec3,
    /// A map of entity metadata, which includes flags and data properties that alter in particular
    /// the way the entity looks. Flags include ones such as 'on fire' and 'sprinting'. The meta
    /// values are indexed by their property key.
    pub entity_metadata: HashMap<u32, EntityDataEntry>,
    /// Specifies if the item was obtained by fishing it up using a fishing rod. It is not clear why
    /// the client needs to know this.
    pub from_fishing: bool,
}

impl PacketType for AddItemActor {
    fn write(&self, writer: &mut Writer) {
        writer.var_i64(self.entity_unique_id);
        writer.var_u64(self.entity_runtime_id);

        self.item.write(writer);

        writer.vec3(self.position);
        writer.vec3(self.velocity);

        writer.entity_metadata(&self.entity_metadata);
        writer.bool(self.from_fishing);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_unique_id: reader.var_i64(),
            entity_runtime_id: reader.var_u64(),

            item: ItemInstance::read(reader),

            position: reader.vec3(),
            velocity: reader.vec3(),

            entity_metadata: reader.entity_metadata(),
            from_fishing: reader.bool(),
        }
    }
}
