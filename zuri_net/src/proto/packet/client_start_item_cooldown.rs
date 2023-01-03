use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent by the client to the server to initiate a cooldown on an item. The purpose of this packet
/// isn't entirely clear.
#[derive(Debug, Clone)]
pub struct ClientStartItemCooldown {
    /// The category of the item to start the cooldown on.
    pub category: String,
    /// The duration of ticks the cooldown should last.
    pub duration: i32,
}

impl PacketType for ClientStartItemCooldown {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.category.as_str());
        writer.var_i32(self.duration);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            category: reader.string(),
            duration: reader.var_i32(),
        }
    }
}
