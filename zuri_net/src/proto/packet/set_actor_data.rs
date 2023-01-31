use std::collections::HashMap;

use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;
use crate::proto::types::entity_data::{EntityDataEntry, EntityProperties};

/// Sent by the server to update the entity metadata of an entity. It includes flags such as if the
/// entity is on fire, but also properties such as the air it has left until it starts drowning.
#[derive(Debug, Clone)]
pub struct SetActorData {
    /// The runtime ID of the entity. The runtime ID is unique for each world session, and entities
    /// are generally identified in packets using this runtime ID.
    pub entity_runtime_id: u64,
    /// A map of entity metadata, which includes flags and data properties that alter in particular
    /// the way the player looks. Flags include ones such as 'on fire' and 'sprinting'. The meta
    /// values are indexed by their property key.
    pub entity_metadata: HashMap<u32, EntityDataEntry>,
    /// A list of properties that the entity inhibits. These properties define specific attributes
    /// of the entity.
    pub entity_properties: EntityProperties,
    /// The server tick at which the packet was sent. It is used in relation to
    /// CorrectPlayerMovePrediction.
    pub tick: u64,
}

impl PacketType for SetActorData {
    fn write(&self, writer: &mut Writer) {
        writer.var_u64(self.entity_runtime_id);
        writer.entity_metadata(&self.entity_metadata);
        self.entity_properties.write(writer);
        writer.var_u64(self.tick);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_runtime_id: reader.var_u64(),
            entity_metadata: reader.entity_metadata(),
            entity_properties: EntityProperties::read(reader),
            tick: reader.var_u64(),
        }
    }
}
