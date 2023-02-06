use zuri_net_derive::packet;
use crate::proto::ints::VarI32;

/// Sent by the client to update multi-player related settings server-side and sent back to online
/// players by the server. The MultiPlayerSettings packet is a Minecraft: Education Edition packet.
/// It has no functionality for the base game.
#[packet]
#[derive(Debug, Clone)]
pub struct MultiPlayerSettings {
    /// The action that should be done when this packet is sent.
    pub action_type: MultiPlayerSettingsAction,
}

#[packet(VarI32)]
#[derive(Debug, Clone)]
pub enum MultiPlayerSettingsAction {
    Enable,
    Disable,
    RefreshJoinCode,
}
