use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::packet::PacketType;
use crate::proto::io::{Reader, Writer};

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum SimpleEventType {
    None,
    CommandsEnabled,
    CommandsDisabled,
    UnlockWorldTemplateSettings,
}

/// Sent by the server to send a 'simple event' to the client, meaning an event without any
/// additional event data. The event is typically used by the client for telemetry.
#[derive(Debug, Clone)]
pub struct SimpleEvent {
    /// The type of the event to be called.
    pub event_type: SimpleEventType,
}

impl PacketType for SimpleEvent {
    fn write(&self, writer: &mut Writer) {
        writer.i16(self.event_type.to_i16().unwrap());
    }

    fn read(reader: &mut Reader) -> Self {
        Self { event_type: SimpleEventType::from_i16(reader.i16()).unwrap() }
    }
}
