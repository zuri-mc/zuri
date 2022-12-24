use crate::io::{Reader, Writer};
use crate::packet::PacketType;

/// Sent by the client to the server when it jumps while riding an entity that has the WASDControlled entity flag set,
/// for example when riding a horse.
#[derive(Debug)]
pub struct PassengerJump {
    /// The strength of the jump, depending on how long the rider has held the jump button.
    pub jump_strength: i32,
}

impl PacketType for PassengerJump {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.jump_strength);
    }

    fn read(reader: &mut Reader) -> Self {
        Self { jump_strength: reader.var_i32() }
    }
}
