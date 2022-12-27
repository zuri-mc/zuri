use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

#[derive(Debug)]
pub struct RemoveObjective {
    pub objective_name: String,
}

impl PacketType for RemoveObjective {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.objective_name.as_str());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            objective_name: reader.string(),
        }
    }
}
