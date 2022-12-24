use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{ToPrimitive, FromPrimitive};

use crate::io::{Reader, Writer};
use crate::packet::PacketType;

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum NPCRequestAction {
    SetActions,
    ExecuteAction,
    ExecuteClosingCommands,
    SetName,
    SetSkin,
    SetInteractText,
    ExecuteOpeningCommands,
}

#[derive(Debug)]
pub struct NPCRequest {
    pub entity_runtime_id: u64,
    pub request_type: NPCRequestAction,
    pub command_string: String,
    pub action_type: u8,
    pub scene_name: String,
}

impl PacketType for NPCRequest {
    fn write(&self, writer: &mut Writer) {
        writer.var_u64(self.entity_runtime_id);
        writer.u8(self.request_type.to_u8().unwrap());
        writer.string(self.command_string.as_str());
        writer.u8(self.action_type);
        writer.string(self.scene_name.as_str());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_runtime_id: reader.var_u64(),
            request_type: NPCRequestAction::from_u8(reader.u8()).unwrap(),
            command_string: reader.string(),
            action_type: reader.u8(),
            scene_name: reader.string(),
        }
    }
}
