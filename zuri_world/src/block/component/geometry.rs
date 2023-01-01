use bevy::prelude::Mesh;
use crate::block::component::Component;

#[derive(Debug)]
pub struct Geometry {
    pub mesh: Mesh,
    // todo: solid faces
}

impl Component for Geometry {}
