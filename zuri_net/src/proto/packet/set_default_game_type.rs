use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;
use crate::proto::types::world::GameType;

/// Sent by the client when it toggles the default game type in the settings UI, and is sent by the
/// server when it actually changes the default game type, resulting in the toggle being changed in
/// the settings UI.
#[derive(Debug, Clone)]
pub struct SetDefaultGameType {
    /// The new game type that is set. When sent by the client, this is the requested new default
    /// game type.
    pub game_type: GameType,
}

impl PacketType for SetDefaultGameType {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.game_type.to_i32().unwrap());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            game_type: GameType::from_i32(reader.var_i32()).unwrap(),
        }
    }
}
