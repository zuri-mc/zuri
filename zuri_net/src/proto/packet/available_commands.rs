use std::iter;

use zuri_net_derive::proto;

use crate::proto::ints::VarU32;
use crate::proto::io::{Readable, Reader, Writable, Writer};
use crate::proto::types::command::{CommandEnum, CommandEnumConstraint};

/// Sent by the server to define a list of all commands that the client can use on the server, along
/// with how to use them.
#[derive(Debug, Clone)]
pub struct AvailableCommands {
    pub enum_values: Vec<String>,
    pub chained_subcommand_values: Vec<String>,
    pub suffixes: Vec<String>,
    pub enums: Vec<CommandEnum>,
    pub chained_subcommands: Vec<ChainedSubcommand>,
    pub commands: Vec<Command>,
    pub dynamic_enums: Vec<DynamicEnum>,
    pub constraints: Vec<CommandEnumConstraint>,
}

#[proto]
#[derive(Debug, Clone)]
pub struct Command {
    pub name: String,
    pub description: String,
    pub flags: u16,
    pub permission_level: u8,
    pub aliases_offset: u32,
    #[len_type(VarU32)]
    pub chained_subcommand_offsets: Vec<u16>,
    #[len_type(VarU32)]
    pub overloads: Vec<CommandOverload>,
}

/// An overload specifies one specific way to use a command.
///
/// Can be compared to operator overloading in languages such as Java or C++. Commands are often
/// given different subcommands by specifying multiple overloads with different signatures and a
/// subcommand name as first parameter. This is not the only use for this however.
#[proto]
#[derive(Debug, Clone)]
pub struct CommandOverload {
    /// If true, the command overload uses chained subcommands.
    pub chaining: bool,
    /// À list of parameters specying the usage of the command when this specific overload is
    /// applied.
    #[len_type(VarU32)]
    pub parameters: Vec<CommandParameter>,
}

/// A single parameter in a command overload. Corresponds to each all possible values accepted at
/// a certain position in the command in this certain overload.
///
/// An example of such a parameter is for instance the choice between `survival`, `creative` and
/// `adventure` mode in the `/gamemode <mode>` command.
#[proto]
#[derive(Debug, Clone)]
pub struct CommandParameter {
    pub name: String,
    pub parameter_type: u32, // todo: give this a type
    pub optional: bool,
    pub options: CommandParameterOption,
}

#[proto(u8)]
#[derive(Debug, Clone, Default)]
pub enum CommandParameterOption {
    #[default]
    None = 0,
    CollapseEnum,
    HasSemanticConstraint,
    AsChainedCommand,
}

#[proto]
#[derive(Debug, Clone)]
pub struct ChainedSubcommand {
    pub name: String,
    #[len_type(VarU32)]
    pub values: Vec<ChainedSubcommandValue>,
}

#[proto]
#[derive(Debug, Clone)]
pub struct ChainedSubcommandValue {
    pub index: u16,
    pub value: u16,
}

#[proto]
#[derive(Debug, Clone)]
pub struct DynamicEnum {
    pub type_name: String,
    pub values: String,
}

impl Readable<AvailableCommands> for AvailableCommands {
    fn read(reader: &mut Reader) -> AvailableCommands {
        let enum_values: Vec<_> = iter::empty::<()>()
            .take(VarU32::read(reader).0 as usize)
            .map(|_| String::read(reader))
            .collect();
        let chained_subcommand_values = iter::empty::<()>()
            .take(VarU32::read(reader).0 as usize)
            .map(|_| String::read(reader))
            .collect();
        let suffixes = iter::empty::<()>()
            .take(VarU32::read(reader).0 as usize)
            .map(|_| String::read(reader))
            .collect();
        let enums = iter::empty::<()>()
            .take(VarU32::read(reader).0 as usize)
            .map(|_| {
                let enum_type = String::read(reader);
                let value_indices_len = VarU32::read(reader).0;
                let mut value_indices = Vec::new();
                for _ in 0..value_indices_len {
                    value_indices.push(if enum_values.len() < (u8::MAX as usize) {
                        reader.u8() as u32
                    } else if enum_values.len() < (u16::MAX as usize) {
                        reader.u16() as u32
                    } else {
                        reader.u32()
                    });
                }
                CommandEnum {
                    enum_type,
                    value_indices,
                }
            })
            .collect();
        let chained_subcommands = iter::empty::<()>()
            .take(VarU32::read(reader).0 as usize)
            .map(|_| ChainedSubcommand::read(reader))
            .collect();
        let commands = iter::empty::<()>()
            .take(VarU32::read(reader).0 as usize)
            .map(|_| Command::read(reader))
            .collect();
        let dynamic_enums = iter::empty::<()>()
            .take(VarU32::read(reader).0 as usize)
            .map(|_| DynamicEnum::read(reader))
            .collect();
        let constraints = iter::empty::<()>()
            .take(VarU32::read(reader).0 as usize)
            .map(|_| CommandEnumConstraint::read(reader))
            .collect();

        Self {
            enum_values,
            chained_subcommand_values,
            suffixes,
            enums,
            chained_subcommands,
            commands,
            dynamic_enums,
            constraints,
        }
    }
}

impl Writable for AvailableCommands {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.enum_values.len() as u32);
        for v in &self.enum_values {
            v.write(writer);
        }
        writer.var_u32(self.chained_subcommand_values.len() as u32);
        for v in &self.chained_subcommand_values {
            v.write(writer);
        }
        writer.var_u32(self.suffixes.len() as u32);
        for v in &self.suffixes {
            v.write(writer);
        }
        writer.var_u32(self.enums.len() as u32);
        for v in &self.enums {
            v.enum_type.write(writer);
            writer.var_u32(v.value_indices.len() as u32);
            for v in v.value_indices.iter().cloned() {
                if self.enum_values.len() < u8::MAX as usize {
                    writer.u8(v as u8);
                } else if self.enum_values.len() < u16::MAX as usize {
                    writer.u16(v as u16);
                } else {
                    writer.u32(v);
                }
            }
        }
        writer.var_u32(self.chained_subcommands.len() as u32);
        for v in &self.chained_subcommands {
            v.write(writer);
        }
        writer.var_u32(self.commands.len() as u32);
        for v in &self.commands {
            v.write(writer);
        }
        writer.var_u32(self.dynamic_enums.len() as u32);
        for v in &self.dynamic_enums {
            v.write(writer);
        }
        writer.var_u32(self.constraints.len() as u32);
        for v in &self.constraints {
            v.write(writer);
        }
    }
}
