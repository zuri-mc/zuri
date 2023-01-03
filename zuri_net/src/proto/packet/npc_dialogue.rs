use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum NPCDialogueAction {
    Open,
    Close,
}

/// Allows the client to display dialog boxes for interacting with NPCs.
#[derive(Debug, Clone)]
pub struct NPCDialogue {
    /// The unique ID of the NPC being requested.
    pub entity_unique_id: u64,
    /// The type of action for the packet.
    pub action_type: NPCDialogueAction,
    /// The dialogue text that the client should see.
    pub dialogue: String,
    /// The identifier of the scene. If this is left empty, the client will use the last scene sent
    /// to it. (https://docs.microsoft.com/en-us/minecraft/creator/documents/npcdialogue)
    pub scene_name: String,
    /// The name of the NPC to be displayed to the client.
    pub npc_name: String,
    /// The JSON string of the buttons/actions the server can perform.
    pub action_json: String,
}

impl PacketType for NPCDialogue {
    fn write(&self, writer: &mut Writer) {
        writer.u64(self.entity_unique_id);
        writer.var_i32(self.action_type.to_i32().unwrap());
        writer.string(self.dialogue.as_str());
        writer.string(self.scene_name.as_str());
        writer.string(self.npc_name.as_str());
        writer.string(self.action_json.as_str());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_unique_id: reader.u64(),
            action_type: NPCDialogueAction::from_i32(reader.var_i32()).unwrap(),
            dialogue: reader.string(),
            scene_name: reader.string(),
            npc_name: reader.string(),
            action_json: reader.string(),
        }
    }
}
