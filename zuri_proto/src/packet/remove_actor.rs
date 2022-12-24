use crate::io::{Reader, Writer};
use crate::packet::PacketType;

/// Sent by the server to remove an entity that currently exists in the world from the client-side. Sending this packet
/// if the client cannot already see this entity will have no effect.
#[derive(Debug)]
pub struct RemoveActor {
    /// The unique ID of the entity to be removed. The unique ID is a value that remains consistent across different
    /// sessions of the same world, but most servers simply fill the runtime ID of the entity out for this field.
    pub entity_unique_id: i64,
}

impl PacketType for RemoveActor {
    fn write(&self, writer: &mut Writer) {
        writer.i64(self.entity_unique_id);
    }

    fn read(reader: &mut Reader) -> Self {
        Self { entity_unique_id: reader.i64() }
    }
}
