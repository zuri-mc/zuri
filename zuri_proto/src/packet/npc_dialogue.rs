#[derive(Debug)]
pub struct NPCDialogue {
    pub entity_unique_id: u64,
    pub action_type: NPCDialogueAction,
    pub dialogue: String,
    pub scene_name: String,
    pub npc_name: String,
    pub action_json: String,
}

impl Packet for NPCDialogue {
    fn write(&self, writer: &mut Writer) {
        writer.u64(self.entity_unique_id);
        writer.var_i32(num::ToPrimitive::to_i32(&self.action_type).unwrap());
        writer.string(self.dialogue.as_str());
        writer.string(self.scene_name.as_str());
        writer.string(self.npc_name.as_str());
        writer.string(self.action_json.as_str());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_unique_id: reader.u64(),
            action_type: num::FromPrimitive::from_i32(reader.var_i32()).unwrap(),
            dialogue: reader.string(),
            scene_name: reader.string(),
            npc_name: reader.string(),
            action_json: reader.string(),
        }
    }
}
