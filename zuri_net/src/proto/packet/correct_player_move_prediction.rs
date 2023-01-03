use glam::Vec3;
use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent by the server if and only if server authoritative movement with rewind is enabled. The
/// packet is used to correct movement at a specific point in time.
#[derive(Debug, Clone)]
pub struct CorrectPlayerMovePrediction {
    /// The position that the player is supposed to be at the tick written in the field below. The
    /// client will change its current position based on movement after that tick starting from the
    /// position.
    pub position: Vec3,
    /// The change in position compared to what the client sent at that specific tick.
    pub delta: Vec3,
    /// Specifies if the player was on the ground at the time of the tick below.
    pub on_ground: bool,
    /// The tick of the movement which was corrected by this packet.
    pub tick: u64,
}

impl PacketType for CorrectPlayerMovePrediction {
    fn write(&self, writer: &mut Writer) {
        writer.vec3(self.position);
        writer.vec3(self.delta);
        writer.bool(self.on_ground);
        writer.var_u64(self.tick);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            position: reader.vec3(),
            delta: reader.vec3(),
            on_ground: reader.bool(),
            tick: reader.var_u64(),
        }
    }
}
