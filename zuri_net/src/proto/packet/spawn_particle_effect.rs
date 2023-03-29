use crate::proto::ints::VarI64;
use bytes::Bytes;
use glam::Vec3;
use zuri_net_derive::proto;

use crate::proto::types::world::Dimension;

/// Sent by the server to spawn a particle effect client-side. Unlike other packets that result in
/// the appearing of particles, this packet can show particles that are not hardcoded in the client.
/// They can be added and changed through behaviour packs to implement custom particles.
#[proto]
#[derive(Debug, Clone)]
pub struct SpawnParticleEffect {
    /// The dimension that the particle is spawned in. Its exact usage is not clear, as the
    /// dimension has no direct effect on the particle.
    #[enum_header(u8)]
    pub dimension: Dimension,
    /// The unique ID of the entity that the spawned particle may be attached to. If this ID is not
    /// negative one, the Position below will be interpreted as relative to the position of the
    /// entity associated with this unique ID.
    pub entity_unique_id: VarI64,
    /// The position that the particle should be spawned at. If the position is too far away from
    /// the player, it will not show up. If `entity_unique_id` is not negative one, the position
    /// will be relative to the position of the entity.
    pub position: Vec3,
    /// The name of the particle that should be shown. This name may point to a particle effect that
    /// is built-in, or to one implemented by behaviour packs.
    pub particle_name: String,
    /// JSON object of MoLang variables that may be applicable to the particle spawn. This can just
    /// be left as `None` in most cases.
    pub molang_variables: Option<Bytes>,
}
