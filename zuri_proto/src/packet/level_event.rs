use glam::Vec3;

use crate::packet::Packet;
use crate::io::{Reader, Writer};
use crate::types::level_event::LevelEventType;

/// Sent by the server to make a certain event in the level occur. It ranges from particles, to
/// sounds, and other events such as starting rain and block breaking.
#[derive(Debug)]
pub struct LevelEvent {
    /// The event that is being 'called'.
    pub event_type: LevelEventType,
    /// The position of the level event. Practically every event requires this Vec3 set for it, as
    /// particles, sounds and block editing relies on it.
    pub position: Vec3,
    /// An integer holding additional data of the event. The type of data held depends on the
    /// EventType.
    pub event_data: i32,
}

impl Packet for LevelEvent {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(num::ToPrimitive::to_i32(&self.event_type).unwrap());
        writer.vec3(self.position);
        writer.var_i32(self.event_data);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            event_type: num::FromPrimitive::from_i32(reader.var_i32()).unwrap(),
            position: reader.vec3(),
            event_data: reader.var_i32(),
        }
    }
}
