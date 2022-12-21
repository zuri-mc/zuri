use bevy::prelude::*;

/// Basic components required by every entity.
#[derive(Bundle)]
pub struct BaseEntityBundle {
    #[bundle]
    pub transform: TransformBundle,
}

/// A component for an entity with a head that has separate rotation from its body.
#[derive(Component, Default)]
pub struct Head {
    /// The rotation of the head relative to the body.
    pub rot: Quat,
    pub eye_height: f32,
}
