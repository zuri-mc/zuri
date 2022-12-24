use crate::io::{Reader, Writer};
use crate::packet::Packet;

#[derive(Debug)]
pub struct CommandRequest {
    pub command_line: String,
    pub command_origin: CommandOrigin,
    pub internal: bool,
}

impl Packet for CommandRequest {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.command_line.as_str());
        self.command_origin.write(writer);
        writer.bool(self.internal);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            command_line: reader.string(),
            command_origin: CommandOrigin::read(reader),
            internal: reader.bool(),
        }
    }
}
