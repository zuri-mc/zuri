use crate::proto::ints::VarI32;
use zuri_net_derive::proto;

/// Allows the client to display dialog boxes for interacting with NPCs.
#[proto]
#[derive(Debug, Clone)]
pub struct NPCDialogue {
    /// The unique ID of the NPC being requested.
    pub entity_unique_id: u64,
    /// The type of action for the packet.
    pub action_type: NPCDialogueAction,
    /// The dialogue text that the client should see.
    pub dialogue: String,
    /// The identifier of the scene. If this is left empty, the client will use the last scene sent
    /// to it. (<https://docs.microsoft.com/en-us/minecraft/creator/documents/npcdialogue>)
    pub scene_name: String,
    /// The name of the NPC to be displayed to the client.
    pub npc_name: String,
    /// The JSON string of the buttons/actions the server can perform.
    pub action_json: String,
}

#[proto(VarI32)]
#[derive(Debug, Clone)]
pub enum NPCDialogueAction {
    Open,
    Close,
}
