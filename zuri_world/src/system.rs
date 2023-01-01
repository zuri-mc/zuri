use bevy::asset::{Assets, Handle};
use bevy::prelude::{Changed, Mesh, Query, Res, ResMut};
use crate::block::component::geometry::Geometry;
use crate::block::RuntimeBlocks;

use crate::chunk::Chunk;

/// Updates the mesh of a chunk when it has been modified.
pub fn chunk_update_system(
    mut assets: ResMut<Assets<Mesh>>,
    mut query: Query<(&mut Handle<Mesh>, &Chunk), Changed<Chunk>>,
    blocks: Res<RuntimeBlocks>,
) {
    for (mesh, chunk) in &mut query {
        assets.set_untracked(mesh.id(), chunk.build_mesh(blocks.components::<Geometry>()));
    }
}
