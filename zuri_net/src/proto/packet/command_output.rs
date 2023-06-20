use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::io::{Readable, Reader, Writable, Writer};
use crate::proto::packet::PacketType;
use crate::proto::types::command::{CommandOrigin, CommandOutputMessage, CommandOutputType};

/// Sent by the server to the client to send text as output of a command. Most servers do not use
/// this packet and instead simply send Text packets, but there is reason to send it. If the origin
/// of a CommandRequest packet is not the player itself, but, for example, a WS server, sending a
/// Text packet will not do what is expected: The message should go to the WS server, not to the
/// client's chat. The CommandOutput packet will make sure the messages are relayed to the correct
/// origin of the command request.
#[derive(Debug, Clone)]
pub struct CommandOutput {
    /// The data specifying the origin of the command. In other words, the source that the command
    /// request was from, such as the player itself or a WS server. The client forwards the messages
    /// in this packet to the right origin, depending on what is sent here.
    pub command_origin: CommandOrigin,
    /// The type of output that is sent. The vanilla game usually sends all output here.
    pub output_type: CommandOutputType,
    /// The amount of times that a command was executed successfully as a result of the command that
    /// was requested. For servers, this is usually a rather meaningless fields, but for vanilla,
    /// this is applicable for commands created with functions.
    pub success_count: u32,
    /// A list of all output messages that should be sent to the player. Whether they are shown or
    /// not, depends on the type of the messages.
    pub output_messages: Vec<CommandOutputMessage>,
    /// The purpose of this field is currently unknown.
    pub data_set: String,
}

impl PacketType for CommandOutput {
    fn write(&self, writer: &mut Writer) {
        self.command_origin.write(writer);
        writer.u8(self.output_type.to_u8().unwrap());
        writer.var_u32(self.success_count);

        writer.var_u32(self.output_messages.len() as u32);
        self.output_messages
            .iter()
            .for_each(|message| message.write(writer));

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
            output_messages: (0..reader.var_u32())
                .map(|_| CommandOutputMessage::read(reader))
                .collect(),
            data_set: if output_type == CommandOutputType::DataSet {
                reader.string()
            } else {
                String::new()
            },
        }
    }
}
