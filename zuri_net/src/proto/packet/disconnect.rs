use crate::proto::ints::VarI32;
use crate::proto::io::{Readable, Reader, Writable, Writer};
use crate::proto::packet::PacketType;

/// Sent by the server to disconnect the client using an optional message to send as the disconnect
/// screen.
#[derive(Debug, Clone)]
pub struct Disconnect {
    /// The reason why the user was kicked. Used for telemetry.
    pub reason: VarI32,
    /// An optional message to show when disconnected. If left empty, the disconnection screen will
    /// be hidden.
    pub message: Option<String>,
}

impl PacketType for Disconnect {
    fn write(&self, writer: &mut Writer) {
        self.reason.write(writer);
        writer.bool(self.message.is_some());
        if self.message.is_some() {
            writer.string(self.message.as_ref().unwrap().as_str());
        }
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            reason: VarI32::read(reader),
            message: if reader.bool() {
                None
            } else {
                Some(reader.string())
            },
        }
    }
}
