use crate::block::component::geometry::Geometry;
use crate::block::BlockMap;
use bevy::asset::{Assets, Handle};
use bevy::math::IVec3;
use bevy::prelude::{Changed, EventReader, Mesh, Query, Res, ResMut};
use zuri_net::proto::packet::update_block::UpdateBlock;

use crate::chunk::{Chunk, ChunkManager};
use crate::pos::ChunkIndex;

/// Updates the mesh of a chunk when it has been modified.
pub(crate) fn chunk_update_system(
    mut assets: ResMut<Assets<Mesh>>,
    mut query: Query<(&mut Handle<Mesh>, &Chunk), Changed<Chunk>>,
    blocks: Res<BlockMap>,
) {
    for (mesh, chunk) in &mut query {
        assets.set_untracked(mesh.id(), chunk.build_mesh(blocks.components::<Geometry>()));
    }
}

/// Updates a block in the world when the server sends a block update.
pub(crate) fn block_update_system(
    mut pks: EventReader<UpdateBlock>,
    chunks: Res<ChunkManager>,
    mut query: Query<&mut Chunk>,
) {
    for pk in pks.iter() {
        // Multi-layer chunks are not yet supported.
        if pk.layer.0 != 0 {
            continue;
        }
        if let Some(chunk_entity) = chunks.at_block_pos(pk.position.into()) {
            query.get_mut(chunk_entity).unwrap().set(
                <ChunkIndex as From<IVec3>>::from(pk.position.into()),
                pk.new_block_runtime_id.into(),
            );
        }
    }
}
