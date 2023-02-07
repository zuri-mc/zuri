use zuri_net_derive::proto;

use crate::proto::types::ability::AbilityData;

/// Sent from the server to update the abilities of the player. It, along with the
/// UpdateAdventureSettings packet, are replacements of the AdventureSettings packet since v1.19.10.
#[proto]
#[derive(Debug, Clone)]
pub struct UpdateAbilities {
    /// Various data about the abilities of a player, such as ability layers or permissions.
    pub ability_data: AbilityData,
}
