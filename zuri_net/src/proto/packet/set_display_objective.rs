use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

#[derive(Debug, Clone)]
pub struct SetDisplayObjective {
    pub display_slot: String,
    pub objective_name: String,
    pub display_name: String,
    pub criteria_name: String,
    pub sort_order: i32,
}

impl PacketType for SetDisplayObjective {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.display_slot.as_str());
        writer.string(self.objective_name.as_str());
        writer.string(self.display_name.as_str());
        writer.string(self.criteria_name.as_str());
        writer.var_i32(self.sort_order);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            display_slot: reader.string(),
            objective_name: reader.string(),
            display_name: reader.string(),
            criteria_name: reader.string(),
            sort_order: reader.var_i32(),
        }
    }
}
