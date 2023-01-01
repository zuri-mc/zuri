use bevy::prelude::*;
use crate::block::build_rids;

use crate::system::chunk_update_system;

pub mod chunk;
pub mod range;
pub mod pos;
pub mod sub_chunk;
mod system;
mod paletted_storage;
pub mod block;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(build_rids()) // temp
            .add_system(chunk_update_system);
    }
}
