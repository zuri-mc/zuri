use zuri_net_derive::proto;

use crate::proto::ints::{VarU32, VarU64};
use crate::proto::types::attribute::Attribute;

/// Sent by the server to update an amount of attributes of any entity in the world. These
/// attributes include ones such as the health or the movement speed of the entity.
#[proto]
#[derive(Debug, Clone)]
pub struct UpdateAttributes {
    /// The runtime ID of the entity. The runtime ID is unique for each world session, and entities
    /// are generally identified in packets using this runtime ID.
    pub entity_runtime_id: VarU64,
    /// A list of new attributes that the entity gets. It includes attributes such as its health,
    /// movement speed, etc. Note that only changed attributes have to be sent in this packet. It is
    /// not required to send attributes that did not have their values changed.
    #[len_type(VarU32)]
    pub attributes: Vec<Attribute>,
    /// The server tick at which the packet was sent. It is used in relation to
    /// CorrectPlayerMovePrediction.
    pub tick: VarU64,
}
