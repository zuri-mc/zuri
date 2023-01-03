use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent by the client in response to a PlayStatus packet with the status set to PlayerSpawn. The
/// packet marks the moment at which the client is fully initialised and can receive any packet
/// without discarding it.
#[derive(Debug, Clone)]
pub struct SetLocalPlayerAsInitialised {
    /// The entity runtime ID the player was assigned earlier in the login sequence in the StartGame
    /// packet.
    pub entity_runtime_id: u64,
}

impl PacketType for SetLocalPlayerAsInitialised {
    fn write(&self, writer: &mut Writer) {
        writer.var_u64(self.entity_runtime_id);
    }

    fn read(reader: &mut Reader) -> Self {
        Self { entity_runtime_id: reader.var_u64() }
    }
}
