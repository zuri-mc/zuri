use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent by the server to enable or disable the ability to execute commands for the client. If
/// disabled, the client itself will stop the execution of commands.
#[derive(Debug, Clone)]
pub struct SetCommandsEnabled {
    /// Defines if the commands should be enabled, or if false, disabled.
    pub enabled: bool,
}

impl PacketType for SetCommandsEnabled {
    fn write(&self, writer: &mut Writer) {
        writer.bool(self.enabled);
    }

    fn read(reader: &mut Reader) -> Self {
        Self { enabled: reader.bool() }
    }
}
