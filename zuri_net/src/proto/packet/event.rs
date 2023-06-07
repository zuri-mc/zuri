use crate::proto::ints::VarU64;
use crate::proto::io::{Readable, Writable};
use crate::proto::types::event::EventType;
use zuri_net_derive::proto;

/// Sent by the server to send an event with additional data. It is typically sent to the client for
/// telemetry reasons, much like the SimpleEvent packet.
#[proto]
#[derive(Debug, Clone)]
pub struct Event {
    /// The runtime ID of the player. The runtime ID is unique for each world session, and entities
    /// are generally identified in packets using this runtime ID.
    pub entity_runtime_id: VarU64,
    /// The parsed event data.
    pub event_data: EventType,
}
