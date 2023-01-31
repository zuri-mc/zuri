use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum NPCRequestAction {
    SetActions,
    ExecuteAction,
    ExecuteClosingCommands,
    SetName,
    SetSkin,
    SetInteractText,
    ExecuteOpeningCommands,
}

/// Sent by the client when it interacts with an NPC. The packet is specifically made for Education
/// Edition, where NPCs are available to use.
#[derive(Debug, Clone)]
pub struct NPCRequest {
    /// The runtime ID of the NPC entity that the player interacted with. It is the same as sent by
    /// the server when spawning the entity.
    pub entity_runtime_id: u64,
    /// The type of the request, which depends on the permission that the player has. It will be
    /// either a type that indicates that the NPC should show its dialog, or that it should open the
    /// editing window.
    pub request_type: NPCRequestAction,
    /// The command string set in the NPC. It may consist of multiple commands, depending on what
    /// the player set in it.
    pub command_string: String,
    /// The type of the action to execute.
    pub action_type: u8,
    /// The name of the scene. This can be left empty to specify the last scene that the player was
    /// sent.
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
