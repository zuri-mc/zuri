use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum MultiPlayerSettingsAction {
    Enable,
    Disable,
    RefreshJoinCode,
}

/// Sent by the client to update multi-player related settings server-side and sent back to online
/// players by the server. The MultiPlayerSettings packet is a Minecraft: Education Edition packet.
/// It has no functionality for the base game.
#[derive(Debug, Clone)]
pub struct MultiPlayerSettings {
    /// The action that should be done when this packet is sent.
    pub action_type: MultiPlayerSettingsAction,
}

impl PacketType for MultiPlayerSettings {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.action_type.to_i32().unwrap());
    }

    fn read(reader: &mut Reader) -> Self {
        Self { action_type: MultiPlayerSettingsAction::from_i32(reader.var_i32()).unwrap() }
    }
}
