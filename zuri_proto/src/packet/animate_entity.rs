use crate::io::{Reader, Writer};
use crate::packet::PacketType;

#[derive(Debug)]
pub struct AnimateEntity {
    pub animation: String,
    pub next_state: String,
    pub stop_condition: String,
    pub stop_condition_version: i32,
    pub controller: String,
    pub blend_out_time: f32,
    pub entity_runtime_ids: Vec<u64>,
}

impl PacketType for AnimateEntity {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.animation.as_str());
        writer.string(self.next_state.as_str());
        writer.string(self.stop_condition.as_str());
        writer.i32(self.stop_condition_version);
        writer.string(self.controller.as_str());
        writer.f32(self.blend_out_time);
        writer.var_u32(self.entity_runtime_ids.len() as u32);
        self.entity_runtime_ids.iter().for_each(|runtime_id| writer.var_u64(*runtime_id));
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            animation: reader.string(),
            next_state: reader.string(),
            stop_condition: reader.string(),
            stop_condition_version: reader.i32(),
            controller: reader.string(),
            blend_out_time: reader.f32(),
            entity_runtime_ids: (0..reader.var_u32()).map(|_| reader.var_u64()).collect(),
        }
    }
}
