use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent by the server to transfer a player from the current server to another. Doing so will fully
/// disconnect the client, bring it back to the main menu and make it connect to the next server.
#[derive(Debug, Clone)]
pub struct Transfer {
    /// The address of the new server, which might be either a hostname or an actual IP address.
    pub address: String,
    /// The UDP port of the new server.
    pub port: u16,
}

impl PacketType for Transfer {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.address.as_str());
        writer.u16(self.port);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            address: reader.string(),
            port: reader.u16(),
        }
    }
}
