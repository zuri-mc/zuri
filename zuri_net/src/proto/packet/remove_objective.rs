use zuri_net_derive::proto;

/// Sent by the server to remove a scoreboard objective. It is used to stop showing a scoreboard to
/// a player.
#[proto]
#[derive(Debug, Clone)]
pub struct RemoveObjective {
    /// The name of the objective that the scoreboard currently active has. This name must be
    /// identical to the one sent in the SetDisplayObjective packet.
    pub objective_name: String,
}
