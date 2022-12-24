use crate::io::{Reader, Writer};
use crate::packet::Packet;

/// Sent by the server to change the client-side velocity of an entity. It is usually used in combination with
/// server-side movement calculation.
#[derive(Debug)]
pub struct SetActorMotion {
    /// The runtime ID of the entity. The runtime ID is unique for each world session, and entities are generally
    /// identified in packets using this runtime ID.
    pub entity_runtime_id: u64,
    /// The new velocity the entity gets. This velocity will initiate the client-side movement of the entity.
    pub velocity: Vec3,
}

impl Packet for SetActorMotion {
    fn write(&self, writer: &mut Writer) {
        writer.var_u64(self.entity_runtime_id);
        writer.vec3(self.velocity);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_runtime_id: reader.var_u64(),
            velocity: reader.vec3(),
        }
    }
}
