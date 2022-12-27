use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

#[derive(Debug, Clone)]
pub struct SettingsCommand {
    pub command_line: String,
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
