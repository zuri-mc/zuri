use bevy::prelude::Mesh;
use zuri_world::block::component::Component;

#[derive(Debug)]
pub struct Geometry {
    pub mesh: Mesh,
    // todo: solid faces
}

impl Component for Geometry {}
