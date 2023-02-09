use zuri_net_derive::proto;

use crate::proto::ints::VarI32;

/// Sent by the client to the server when it jumps while riding an entity that has the
/// WASDControlled entity flag set, for example when riding a horse.
#[proto]
#[derive(Debug, Clone)]
pub struct PassengerJump {
    /// The strength of the jump, depending on how long the rider has held the jump button.
    pub jump_strength: VarI32,
}
