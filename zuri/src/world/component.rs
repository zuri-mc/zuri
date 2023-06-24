use bevy::prelude::Mesh;
use zuri_world::block;

/// Contains a block's geometry.
///
/// todo: actually load geometry for all blocks
#[derive(Debug, block::Component)]
pub struct Geometry {
    pub mesh: Mesh,
}
