use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;
use crate::proto::types::command::CommandOrigin;

#[derive(Debug)]
pub struct CommandRequest {
    pub command_line: String,
    pub command_origin: CommandOrigin,
    pub internal: bool,
}

impl PacketType for CommandRequest {
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
