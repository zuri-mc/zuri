use zuri_net_derive::proto;

use crate::proto::ints::VarU32;

/// Sent by the server to render the different fogs in the Stack. The types of fog are controlled by
/// resource packs to change how they are rendered, and the ability to create custom fog.
#[proto]
#[derive(Debug, Clone)]
pub struct PlayerFog {
    /// A list of fog identifiers to be sent to the client. Examples of fog identifiers are
    /// "minecraft:fog_ocean" and "minecraft:fog_hell".
    #[len_type(VarU32)]
    pub stack: Vec<String>,
}
