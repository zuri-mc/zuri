use bevy::prelude::*;

/// Basic components required by every entity.
#[derive(Bundle)]
pub struct BaseEntityBundle {
    pub rid: RuntimeId,
    #[bundle]
    pub transform: TransformBundle,
}

/// The server-side identifier of an entity.
#[derive(Component)]
pub struct RuntimeId(pub u64);

/// A component for an entity with a head that has separate rotation from its body.
#[derive(Component, Default)]
pub struct Head {
    /// The rotation of the head relative to the body.
    pub rot: Quat,
    pub eye_height: f32,
}

impl Head {
    pub fn new(pitch: f32, eye_height: f32) -> Head {
        Head {
            rot: Quat::from_rotation_x(pitch),
            eye_height,
        }
    }
}
