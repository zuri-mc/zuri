use num_traits::{FromPrimitive, ToPrimitive};

use crate::packet::Packet;
use crate::io::{Reader, Writer};
use crate::types::command::{CommandOrigin, CommandOutputMessage, CommandOutputType};

#[derive(Debug)]
pub struct CommandOutput {
    pub command_origin: CommandOrigin,
    pub output_type: CommandOutputType,
    pub success_count: u32,
    pub output_messages: Vec<CommandOutputMessage>,
    pub data_set: String,
}

impl Packet for CommandOutput {
    fn write(&self, writer: &mut Writer) {
        self.command_origin.write(writer);
        writer.u8(self.output_type.to_u8().unwrap());
        writer.var_u32(self.success_count);

        writer.var_u32(self.output_messages.len() as u32);
        self.output_messages.iter().for_each(|message| message.write(writer));

        if self.output_type == CommandOutputType::DataSet {
            writer.string(self.data_set.as_str());
        }
    }

    fn read(reader: &mut Reader) -> Self {
        let command_origin = CommandOrigin::read(reader);
        let output_type = CommandOutputType::from_u8(reader.u8()).unwrap();
        Self {
            command_origin,
            output_type,
            success_count: reader.var_u32(),
            output_messages: (0..reader.var_u32()).map(|_| CommandOutputMessage::read(reader)).collect(),
            data_set: if output_type == CommandOutputType::DataSet { reader.string() } else { "".to_string() },
        }
    }
}
