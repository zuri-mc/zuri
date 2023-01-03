use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent by the server to remove a scoreboard objective. It is used to stop showing a scoreboard to
/// a player.
#[derive(Debug, Clone)]
pub struct RemoveObjective {
    /// The name of the objective that the scoreboard currently active has. This name must be
    /// identical to the one sent in the SetDisplayObjective packet.
    pub objective_name: String,
}

impl PacketType for RemoveObjective {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.objective_name.as_str());
    }

    fn read(reader: &mut Reader) -> Self {
        Self { objective_name: reader.string() }
    }
}
