use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent by the server to display an object as a scoreboard to the player. Once sent, it should be
/// followed up by a SetScore packet to set the lines of the packet.
#[derive(Debug, Clone)]
pub struct SetDisplayObjective {
    /// The slot in which the scoreboard should be displayed.
    pub display_slot: String,
    /// The name of the objective that the scoreboard displays. Filling out a random unique value
    /// for this field works: It is not displayed in the scoreboard.
    pub objective_name: String,
    /// The name, or title, that is displayed at the top of the scoreboard.
    pub display_name: String,
    /// The name of the criteria that need to be fulfilled in order for the score to be increased.
    /// This can be any kind of string and does not show up client-side.
    pub criteria_name: String,
    /// The order in which entries on the scoreboard should be sorted.
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
