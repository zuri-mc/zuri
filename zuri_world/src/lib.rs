use crate::block::build_rids;
use bevy::prelude::*;

use crate::system::{block_update_system, chunk_update_system};

pub mod block;
pub mod chunk;
mod paletted_storage;
pub mod pos;
pub mod range;
pub(crate) mod sub_chunk;
mod system;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(build_rids()) // temp
            .add_system_to_stage(CoreStage::PostUpdate, chunk_update_system)
            .add_system_to_stage(CoreStage::PreUpdate, block_update_system);
    }
}
