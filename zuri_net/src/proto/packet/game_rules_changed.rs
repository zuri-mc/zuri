use crate::proto::ints::VarU32;
use crate::proto::types::game_rule::GameRule;
use zuri_net_derive::proto;

/// Sent by the server to the client to update client-side game rules, such as game rules like the
/// 'showCoordinates' game rule.
#[proto]
#[derive(Debug, Clone)]
pub struct GameRulesChanged {
    /// Defines game rules changed with their respective values. The value of these game rules may
    /// be either 'bool', 'i32' or 'f32'. Note that some game rules are server side only, and don't
    /// necessarily need to be sent to the client. Only changed game rules need to be sent in this
    /// packet. Game rules that were not changed do not need to be sent if the client is already
    /// updated on them.
    #[len_type(VarU32)]
    pub game_rules: Vec<GameRule>,
}
