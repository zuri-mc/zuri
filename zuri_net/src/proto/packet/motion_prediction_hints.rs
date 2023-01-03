use glam::Vec3;

use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent by the server to the client. There is a predictive movement component for entities. This
/// packet fills the "history" of that component and entity movement is computed based on the
/// points. Vanilla sends this packet instead of the SetActorMotion packet when 'spatial
/// optimisations' are enabled.
#[derive(Debug, Clone)]
pub struct MotionPredictionHints {
    /// The runtime ID of the entity whose velocity is sent to the client.
    pub entity_runtime_id: u64,
    /// The server-calculated velocity of the entity at the point of sending the packet.
    pub velocity: Vec3,
    /// Specifies if the server currently thinks the entity is on the ground.
    pub on_ground: bool,
}

impl PacketType for MotionPredictionHints {
    fn write(&self, writer: &mut Writer) {
        writer.var_u64(self.entity_runtime_id);
        writer.vec3(self.velocity);
        writer.bool(self.on_ground);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_runtime_id: reader.var_u64(),
            velocity: reader.vec3(),
            on_ground: reader.bool(),
        }
    }
}
