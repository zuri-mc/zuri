use crate::block::component::Component;
use bevy::prelude::Mesh;

#[derive(Debug)]
pub struct Geometry {
    pub mesh: Mesh,
    // todo: solid faces
}

impl Component for Geometry {}
