use crate::io::{Reader, Writer};
use crate::packet::Packet;

/// Sent by the server when a particular event happens that has to do with an entity. Some of these events are
/// entity-specific, for example a wolf shaking itself dry, but others are used for each entity, such as dying.
#[derive(Debug)]
pub struct ActorEvent {
    /// The runtime ID of the entity. The runtime ID is unique for each world session, and entities are generally
    /// identified in packets using this runtime ID.
    pub entity_runtime_id: u64,
    /// The type of event to be called.
    pub event_type: ActorEventType,
    /// Optional data associated with a particular event. The data has a different function for different events,
    /// however most events don't use this field at all.
    pub event_data: i32,
}

impl Packet for ActorEvent {
    fn write(&self, writer: &mut Writer) {
        writer.var_u64(self.entity_runtime_id);
        writer.u8(num::ToPrimitive::to_u8(&self.event_type).unwrap());
        writer.var_i32(self.event_data);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_runtime_id: reader.var_u64(),
            event_type: num::FromPrimitive::from_u8(reader.u8()).unwrap(),
            event_data: reader.var_i32(),
        }
    }
}
