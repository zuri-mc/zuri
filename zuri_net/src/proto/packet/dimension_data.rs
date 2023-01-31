use zuri_net_derive::packet;
use crate::proto::ints::VarU32;
use crate::proto::io::{Readable, Writable};
use crate::proto::types::world::DimensionDefinition;

/// A packet sent from the server to the client containing information about data-driven dimensions
/// that the server may have registered. This packet does not seem to be sent by default, rather
/// only being sent when any data-driven dimensions are registered.
#[packet]
#[derive(Debug, Clone)]
pub struct DimensionData {
    /// A list of data-driven dimension definitions registered on the server.
    #[size_type(VarU32)]
    pub definitions: Vec<DimensionDefinition>,
}
