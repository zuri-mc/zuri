use crate::proto::ints::VarI32;
use bytes::Bytes;
use zuri_net_derive::proto;

#[proto(VarI32)]
#[derive(Debug, Clone)]
pub enum AgentActionType {
    None,
    Attack,
    Collect,
    Destroy,
    DetectRedstone,
    DetectObstacle,
    Drop,
    DropAll,
    Inspect,
    InspectData,
    InspectItemCount,
    InspectItemDetail,
    InspectItemSpace,
    Interact,
    Move,
    PlaceBlock,
    Till,
    TransferItemTo,
    Turn,
}

/// An Education Edition packet sent from the server to the client to return a response to a
/// previously requested action.
#[proto]
#[derive(Debug, Clone)]
pub struct AgentAction {
    /// JSON identifier referenced in the initial action.
    pub identifier: String,
    /// The action type that was requested.
    pub action: AgentActionType,
    /// JSON containing the response to the action.
    pub response: Bytes,
}
