use zuri_net_derive::proto;

use crate::proto::ints::VarI32;

/// Sent by the server to display an object as a scoreboard to the player. Once sent, it should be
/// followed up by a SetScore packet to set the lines of the packet.
#[proto]
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
    pub sort_order: VarI32,
}
