use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;
use crate::proto::types::game_rule::GameRule;

/// Sent by the server to the client to update client-side game rules, such as game rules like the
/// 'showCoordinates' game rule.
#[derive(Debug, Clone)]
pub struct GameRulesChanged {
    /// Defines game rules changed with their respective values. The value of these game rules may
    /// be either 'bool', 'i32' or 'f32'. Note that some game rules are server side only, and don't
    /// necessarily need to be sent to the client. Only changed game rules need to be sent in this
    /// packet. Game rules that were not changed do not need to be sent if the client is already
    /// updated on them.
    pub game_rules: Vec<GameRule>,
}

impl PacketType for GameRulesChanged {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.game_rules.len() as u32);
        self.game_rules.iter().for_each(|game_rule| game_rule.write(writer));
    }

    fn read(reader: &mut Reader) -> Self {
        Self { game_rules: (0..reader.var_u32()).map(|_| GameRule::read(reader)).collect() }
    }
}
