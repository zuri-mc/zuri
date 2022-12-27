use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

#[derive(Debug)]
pub struct AddBehaviourTree {
    pub behaviour_tree: String,
}

impl PacketType for AddBehaviourTree {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.behaviour_tree.as_str());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            behaviour_tree: reader.string(),
        }
    }
}
