use crate::io::{Reader, Writer};
use crate::packet::Packet;

#[derive(Debug)]
pub struct SettingsCommand {
    pub command_line: String,
    pub suppress_output: bool,
}

impl Packet for SettingsCommand {
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
