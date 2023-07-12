#![allow(deprecated)]

use zuri_net_derive::proto;

use crate::proto::types::ability::AbilityData;

/// Functions the same as UpdateAbilities. It is unclear why these two are separated.
#[proto]
#[derive(Debug, Clone)]
#[deprecated = "Deprecated as of Bedrock Edition v1.20.10"]
pub struct ClientCheatAbility {
    /// Various data about the abilities of a player, such as ability layers or permissions.
    pub ability_data: AbilityData,
}
