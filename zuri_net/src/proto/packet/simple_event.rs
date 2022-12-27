use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

#[derive(Debug, Clone)]
pub struct SimpleEvent {
    pub event_type: SimpleEventType,
}

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum SimpleEventType {
    None,
    CommandsEnabled,
    CommandsDisabled,
    UnlockWorldTemplateSettings,
}

impl PacketType for SimpleEvent {
    fn write(&self, writer: &mut Writer) {
        writer.i16(self.event_type.to_i16().unwrap());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            event_type: SimpleEventType::from_i16(reader.i16()).unwrap(),
        }
    }
}
