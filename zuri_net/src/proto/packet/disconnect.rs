use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent by the server to disconnect the client using an optional message to send as the disconnect
/// screen.
#[derive(Debug, Clone)]
pub struct Disconnect {
    /// An optional message to show when disconnected. If left empty, the disconnection screen will
    /// be hidden.
    pub message: Option<String>,
}

impl PacketType for Disconnect {
    fn write(&self, writer: &mut Writer) {
        writer.bool(self.message.is_some());
        if self.message.is_some() {
            writer.string(self.message.as_ref().unwrap().as_str());
        }
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            message: if reader.bool() {
                None
            } else {
                Some(reader.string())
            },
        }
    }
}
