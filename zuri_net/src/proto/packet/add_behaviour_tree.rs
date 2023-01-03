use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent by the server to the client. The packet is currently unused by both client and server.
#[derive(Debug, Clone)]
pub struct AddBehaviourTree {
    /// An unused string.
    pub behaviour_tree: String,
}

impl PacketType for AddBehaviourTree {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.behaviour_tree.as_str());
    }

    fn read(reader: &mut Reader) -> Self {
        Self { behaviour_tree: reader.string() }
    }
}
