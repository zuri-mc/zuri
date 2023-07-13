use num_derive::{FromPrimitive, ToPrimitive};
use uuid::Uuid;

use zuri_net_derive::proto;

use crate::proto::ints::VarU32;

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

#[derive(Debug, Clone, Default)]
pub struct CommandEnum {
    pub enum_type: String,
    pub value_indices: Vec<u32>,
}

#[proto]
#[derive(Debug, Clone)]
pub struct CommandEnumConstraint {
    pub enum_option: String,
    pub enum_name: String,
    #[len_type(VarU32)]
    pub constraints: Vec<CommandEnumConstraints>,
}

#[proto(u8)]
#[derive(Debug, Clone)]
pub enum CommandEnumConstraints {
    CheatsEnabled,
    OperatorPermissions,
    HostPermissions,
}

#[proto]
#[derive(Debug, Clone)]
pub struct CommandOrigin {
    pub origin: CommandOriginType,
    pub uuid: Uuid,
    pub request_id: String,
    pub player_unique_id: i64,
}

#[proto]
#[derive(Debug, Clone)]
pub struct CommandOutputMessage {
    pub success: bool,
    pub message: String,
    #[len_type(VarU32)]
    pub parameters: Vec<String>,
}
