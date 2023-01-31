use glam::Vec3;

use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent by the server to the client to make a painting entity show up. It is one of the few
/// entities that cannot be sent using the AddActor packet.
#[derive(Debug, Clone)]
pub struct AddPainting {
    /// The unique ID of the entity. The unique ID is a value that remains consistent across
    /// different sessions of the same world, but most servers simply fill the runtime ID of the
    /// entity out for this field.
    pub entity_unique_id: i64,
    /// The runtime ID of the entity. The runtime ID is unique for each world session, and entities
    /// are generally identified in packets using this runtime ID.
    pub entity_runtime_id: u64,
    /// The position to spawn the entity on. If the entity is on a distance that the player cannot
    /// see it, the entity will still show up if the player moves closer.
    pub position: Vec3,
    /// The facing direction of the painting.
    pub direction: i32,
    /// The title of the painting. It specifies the motive of the painting. The title of the
    /// painting must be valid.
    pub title: String,
}

impl PacketType for AddPainting {
    fn write(&self, writer: &mut Writer) {
        writer.var_i64(self.entity_unique_id);
        writer.var_u64(self.entity_runtime_id);

        writer.vec3(self.position);
        writer.var_i32(self.direction);
        writer.string(self.title.as_str());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_unique_id: reader.var_i64(),
            entity_runtime_id: reader.var_u64(),

            position: reader.vec3(),
            direction: reader.var_i32(),
            title: reader.string(),
        }
    }
}
