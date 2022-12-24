/// Sent by the server to send a player animation from one player to all viewers of that player. It is used for a couple
/// of actions, such as arm swimming and critical hits.
#[derive(Debug)]
pub struct Animate {
    /// The action type to execute.
    pub action_type: AnimateAction,
    /// The runtime ID of the player that the animation should be played upon. The runtime ID is unique for each world
    /// session, and entities are generally identified in packets using this runtime ID.
    pub entity_runtime_id: u64,
    /// It is unclear what this field does.
    pub boat_rowing_time: f32,
}

impl Packet for Animate {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(num::ToPrimitive::to_i32(&self.action_type).unwrap());
        writer.var_u64(self.entity_runtime_id);
        match self.action_type {
            AnimateAction::RowRight | AnimateAction::RowLeft => {
                writer.f32(self.boat_rowing_time);
            }
            _ => {}
        }
    }

    fn read(reader: &mut Reader) -> Self {
        let action_type = num::FromPrimitive::from_i32(reader.var_i32()).unwrap();
        Self {
            action_type,
            entity_runtime_id: reader.var_u64(),
            boat_rowing_time: if action_type == AnimateAction::RowRight || action_type == AnimateAction::RowLeft {
                reader.f32()
            } else {
                0.0
            },
        }
    }
}