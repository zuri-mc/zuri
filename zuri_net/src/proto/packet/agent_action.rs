use bytes::Bytes;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
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
#[derive(Debug, Clone)]
pub struct AgentAction {
    /// JSON identifier referenced in the initial action.
    pub identifier: String,
    /// The action type that was requested.
    pub action: AgentActionType,
    /// JSON containing the response to the action.
    pub response: Bytes,
}

impl PacketType for AgentAction {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.identifier.as_str());
        writer.var_i32(self.action.to_i32().unwrap());
        writer.byte_slice(&self.response);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            identifier: reader.string(),
            action: AgentActionType::from_i32(reader.var_i32()).unwrap(),
            response: reader.byte_slice(),
        }
    }
}
