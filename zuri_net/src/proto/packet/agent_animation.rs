use crate::proto::ints::VarU64;
use zuri_net_derive::proto;

/// An Education Edition packet sent from the server to the client to return a response to a
/// previously requested action.
#[proto]
#[derive(Debug, Clone)]
pub struct AgentAnimation {
    /// The ID of the animation that the agent should perform. As of its implementation, there are
    /// no IDs that can be used in the regular client.
    pub animation: u8,
    /// The runtime ID of the target entity.
    pub entity_runtime_id: VarU64,
}
