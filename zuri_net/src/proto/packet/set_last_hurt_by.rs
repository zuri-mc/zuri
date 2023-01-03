use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent by the server to let the client know what entity type it was last hurt by. At this moment,
/// the packet is useless and should not be used. There is no behaviour that depends on if this
/// packet is sent or not.
#[derive(Debug, Clone)]
pub struct SetLastHurtBy {
    /// The numerical type of the entity that the player was last hurt by.
    pub entity_type: i32,
}

impl PacketType for SetLastHurtBy {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.entity_type);
    }

    fn read(reader: &mut Reader) -> Self {
        Self { entity_type: reader.var_i32() }
    }
}
