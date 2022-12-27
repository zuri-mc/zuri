use bevy::prelude::*;

use crate::system::chunk_update_system;

pub mod chunk;
pub mod range;
pub mod pos;
pub mod sub_chunk;
mod system;
mod paletted_storage;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(chunk_update_system);
    }
}
