#[derive(Debug)]
pub struct AddBehaviourTree {
    pub behaviour_tree: String,
}

impl Packet for AddBehaviourTree {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.behaviour_tree.as_str());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            behaviour_tree: reader.string(),
        }
    }
}
