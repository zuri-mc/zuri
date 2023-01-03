use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent by the client when it changes a setting in the settings that results in the issuing of a
/// command to the server, such as when Show Coordinates is enabled.
#[derive(Debug, Clone)]
pub struct SettingsCommand {
    /// The full command line that was sent to the server as a result of the setting that the client
    /// changed.
    pub command_line: String,
    /// Specifies if the client requests the suppressing of the output of the command that was
    /// executed. Generally this is set to true, as the client won't need a message to confirm the
    /// output of the change.
    pub suppress_output: bool,
}

impl PacketType for SettingsCommand {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.command_line.as_str());
        writer.bool(self.suppress_output);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            command_line: reader.string(),
            suppress_output: reader.bool(),
        }
    }
}
