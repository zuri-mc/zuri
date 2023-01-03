use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent by the server to the client. Its function is not entirely clear: It does not remove an
/// entity in the sense of an in-game entity, but has to do with the ECS that Minecraft uses.
#[derive(Debug, Clone)]
pub struct RemoveEntity {
    /// The network ID of the entity that should be removed.
    pub entity_network_id: u64,
}

impl PacketType for RemoveEntity {
    fn write(&self, writer: &mut Writer) {
        writer.var_u64(self.entity_network_id);
    }

    fn read(reader: &mut Reader) -> Self {
        Self { entity_network_id: reader.var_u64() }
    }
}
