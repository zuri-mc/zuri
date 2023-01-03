use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent from the server to the client expected to be sent when a player dies. It contains messages
/// related to the player's death, which are shown on the death screen as of v1.19.10.
#[derive(Debug, Clone)]
pub struct DeathInfo {
    /// The cause of the player's death, such as "suffocation" or "suicide".
    pub cause: String,
    /// A list of death messages to be shown on the death screen.
    pub messages: Vec<String>,
}

impl PacketType for DeathInfo {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.cause.as_str());
        writer.var_u32(self.messages.len() as u32);
        self.messages.iter().for_each(|m| writer.string(m.as_str()));
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            cause: reader.string(),
            messages: (0..reader.var_u32()).map(|_| reader.string()).collect(),
        }
    }
}
