use std::collections::BTreeMap;

use bytes::Bytes;
use num_derive::{FromPrimitive, ToPrimitive};
use uuid::Uuid;
use zuri_net_derive::proto;
use crate::proto::ints::VarU32;

use crate::proto::io::{Reader, Writer};

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum CommandArg {
    TypeInt = 1,
    TypeFloat = 3,
    TypeValue = 4,
    TypeWildcardInt = 5,
    TypeOperator = 6,
    TypeCompareOperator = 7,
    TypeTarget = 8,
    TypeWildcardTarget = 10,
    TypeFilepath = 17,
    TypeIntegerRange = 23,
    TypeEquipmentSlots = 38,
    TypeString = 39,
    TypeBlockPosition = 47,
    TypePosition = 48,
    TypeMessage = 51,
    TypeRawText = 53,
    TypeJSON = 57,
    TypeBlockStates = 67,
    TypeCommand = 70,

    Valid = 0x100000,
    Enum = 0x200000,
    Suffixed = 0x1000000,
    SoftEnum = 0x4000000,
}

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum CommandConstraint {
    CheatsEnabled,
    OperatorPermissions,
    HostPermissions,
}

#[proto(VarU32)]
#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum CommandOriginType {
    Player,
    Block,
    MinecartBlock,
    DevConsole,
    Test,
    AutomationPlayer,
    ClientAutomation,
    DedicatedServer,
    Entity,
    Virtual,
    GameArgument,
    EntityServer,
    Precompiled,
    GameDirectorEntityServer,
    Script,
    Executor,
}

#[proto(u8)]
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
pub enum CommandOutputType {
    None,
    LastOutput,
    Silent,
    AllOutput,
    DataSet,
}

#[proto(VarU32)]
#[derive(Debug, Copy, Clone, FromPrimitive, ToPrimitive)]
pub enum CommandPermissionLevel {
    Normal,
    GameDirectors,
    Admin,
    Host,
    Owner,
    Internal,
}

#[proto(u8)]
#[derive(Debug, Clone)]
pub enum SoftEnumAction {
    Add,
    Remove,
    Set,
}

// todo: figure out where this is used
#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum ParamOption {
    None,
    CollapseEnum,
    HasSemanticConstraint,
    AsChainedCommand,
}

#[derive(Debug, Clone)]
pub struct Command {
    pub name: String,
    pub description: String,
    pub flags: u16,
    pub permission_level: u8,
    pub aliases: Vec<String>,
    pub overloads: Vec<CommandOverload>,
}

impl Command {
    pub fn write(&self, _: &mut Writer) {
        // writer.string(self.name.as_str());
        // writer.string(self.description.as_str());
        // writer.u16(self.flags);
        // writer.u8(self.permission_level);
        // writer.write_TODO(self.LEN);
        // writer.write_String(self.aliases);
        // writer.write_TODO(self.LEN);
        // writer.write_CommandOverload(self.overloads);
        todo!()
    }

    pub fn read(_: &mut Reader) -> Self {
        // Self {
        //     name: reader.string(),
        //     description: reader.string(),
        //     flags: reader.u16(),
        //     permission_level: reader.u8(),
        //     LEN: reader.read_TODO(),
        //     aliases: reader.read_String(),
        //     LEN: reader.read_TODO(),
        //     overloads: reader.read_CommandOverload(),
        // }
        todo!()
    }
}

#[derive(Debug, Clone, Default)]
pub struct CommandEnum {
    pub enum_type: String,
    pub options: Vec<String>,
    pub dynamic: bool,
}

impl CommandEnum {
    pub fn write(&self, writer: &mut Writer, value_indices: BTreeMap<String, usize>) {
        writer.string(self.enum_type.as_str());
        writer.var_u32(self.options.len() as u32);
        if self.dynamic {
            self.options.iter().for_each(|option| writer.string(option.as_str()));
        } else {
            let len = value_indices.len();
            if len <= u8::MAX as usize {
                self.options.iter().for_each(|option| writer.u8(*value_indices.get(option).unwrap() as u8));
            } else if len <= u16::MAX as usize {
                self.options.iter().for_each(|option| writer.u16(*value_indices.get(option).unwrap() as u16));
            } else {
                self.options.iter().for_each(|option| writer.u32(*value_indices.get(option).unwrap() as u32));
            }
        }
    }

    pub fn read(reader: &mut Reader) -> Self {
        let command_enum = Self {
            enum_type: reader.string(),
            ..Default::default()
        };
        // TODO: READING

        command_enum
    }
}

#[derive(Debug, Clone)]
pub struct CommandEnumConstraint {
    pub enum_option: String,
    pub enum_name: String,
    pub constraints: Bytes,
}

impl CommandEnumConstraint {
    pub fn write(&self, writer: &mut Writer) {
        writer.string(self.enum_option.as_str());
        writer.string(self.enum_name.as_str());
        writer.byte_slice(&self.constraints);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            enum_option: reader.string(),
            enum_name: reader.string(),
            constraints: reader.byte_slice(),
        }
    }
}

#[proto]
#[derive(Debug, Clone)]
pub struct CommandOrigin {
    pub origin: CommandOriginType,
    pub uuid: Uuid,
    pub request_id: String,
    pub player_unique_id: i64,
}

#[derive(Debug, Clone)]
pub struct CommandOutputMessage {
    pub success: bool,
    pub message: String,
    pub parameters: Vec<String>,
}

impl CommandOutputMessage {
    pub fn write(&self, writer: &mut Writer) {
        writer.bool(self.success);
        writer.string(self.message.as_str());
        writer.var_u32(self.parameters.len() as u32);
        self.parameters.iter().for_each(|parameter| writer.string(parameter.as_str()));
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            success: reader.bool(),
            message: reader.string(),
            parameters: (0..reader.var_u32()).map(|_| reader.string()).collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CommandOverload {
    pub parameters: Vec<CommandParameter>,
}

impl CommandOverload {
    pub fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.parameters.len() as u32);
        self.parameters.iter().for_each(|parameter| parameter.write(writer));
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            parameters: (0..reader.var_u32()).map(|_| CommandParameter::read(reader)).collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CommandParameter {
    pub name: String,
    pub parameter_type: u32,
    pub optional: bool,
    pub options: u8,
    pub command_enum: CommandEnum,
    pub suffix: String,
}

impl CommandParameter {
    pub fn write(&self, writer: &mut Writer) {
        // if self.command_enum.dynamic {
        //     self.parameter_type = CommandArg::SoftEnum | CommandArg::Valid |
        // }
        writer.string(self.name.as_str());
        writer.u32(self.parameter_type);
        writer.bool(self.optional);
        writer.u8(self.options);
        todo!()
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            name: reader.string(),
            parameter_type: reader.u32(),
            optional: reader.bool(),
            options: reader.u8(),
            command_enum: CommandEnum::read(reader),
            suffix: reader.string(),
        }
    }
}
