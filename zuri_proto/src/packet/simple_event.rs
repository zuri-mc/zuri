use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use crate::io::{Reader, Writer};
use crate::packet::PacketType;

#[derive(Debug)]
pub struct SimpleEvent {
    pub event_type: SimpleEventType,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
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
