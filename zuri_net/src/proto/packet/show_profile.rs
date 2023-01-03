use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent by the server to show the XBOX Live profile of one player to another.
#[derive(Debug, Clone)]
pub struct ShowProfile {
    /// The XBOX Live User ID of the player whose profile should be shown to the player. If it is
    /// not a valid XUID, the client ignores the packet.
    pub xuid: String,
}

impl PacketType for ShowProfile {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.xuid.as_str());
    }

    fn read(reader: &mut Reader) -> Self {
        Self { xuid: reader.string() }
    }
}
