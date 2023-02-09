use glam::Vec3;

use zuri_net_derive::proto;

use crate::proto::ints::VarU64;

/// Sent by the server to the client. There is a predictive movement component for entities. This
/// packet fills the "history" of that component and entity movement is computed based on the
/// points. Vanilla sends this packet instead of the SetActorMotion packet when 'spatial
/// optimisations' are enabled.
#[proto]
#[derive(Debug, Clone)]
pub struct MotionPredictionHints {
    /// The runtime ID of the entity whose velocity is sent to the client.
    pub entity_runtime_id: VarU64,
    /// The server-calculated velocity of the entity at the point of sending the packet.
    pub velocity: Vec3,
    /// Specifies if the server currently thinks the entity is on the ground.
    pub on_ground: bool,
}
