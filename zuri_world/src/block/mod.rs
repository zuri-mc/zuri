pub mod types;
pub mod component;

use std::any::TypeId;
use std::collections::HashMap;
use std::fmt::Debug;
use bevy::prelude::{Mesh, Resource};
use bevy::render::mesh::PrimitiveTopology;
use bevy_render::mesh::Indices;
use crate::block::component::*;
use crate::block::component::geometry::Geometry;

// todo: remove this temporary function. we want to eventually build all the runtime IDs from data
pub fn build_rids() -> RuntimeBlocks {
    let mut rids = RuntimeBlocks::new();
    rids.extend(16000);
    rids.add_component_type::<Geometry>(ComponentStorageType::Vector);
    // air
    rids.set_component(10462, Geometry {
        mesh: Mesh::new(PrimitiveTopology::TriangleList),
    });
    // grass (the non full block one)
    {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        let vertices: Vec<[f32; 3]> = vec![
            [0., 0., 0.],
            [0., 1., 0.],
            [1., 0., 1.],
            [1., 1., 1.],
        ];
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        let normals: Vec<[f32; 3]> = vec![
            [1., 0., 0.],
            [1., 0., 0.],
            [1., 0., 0.],
            [1., 0., 0.],
        ];
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        let uvs: Vec<[f32; 2]> = vec![
            [0., 0.],
            [0., 1.],
            [1., 0.],
            [1., 1.],
        ];
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.set_indices(Some(Indices::U32(vec![
            0, 1, 2,
            1, 2, 3,
            2, 1, 0,
            3, 2, 1,
        ])));

        rids.set_component(1551, Geometry {
            mesh,
        });
    }

    rids
}

#[derive(Resource)]
pub struct RuntimeBlocks {
    size: u32,
    components: HashMap<TypeId, Box<dyn IComponentStorage>>,
}

impl RuntimeBlocks {
    pub fn new() -> RuntimeBlocks {
        Self {
            size: 0,
            components: Default::default(),
        }
    }

    pub fn component<T: Component>(&self, index: u32) -> Option<&T> {
        self.components::<T>().get(index)
    }

    pub fn components<T: Component>(&self) -> &ComponentStorage<T> {
        self.components.get(&TypeId::of::<T>()).expect("Component not registered").downcast_ref().unwrap()
    }

    pub fn components_mut<T: Component>(&mut self) -> &mut ComponentStorage<T> {
        self.components.get_mut(&TypeId::of::<T>()).expect("Component not registered").downcast_mut().unwrap()
    }

    pub fn set_component<T: Component>(&mut self, index: u32, value: T) {
        self.components_mut::<T>().set(index, value);
    }

    pub fn extend(&mut self, additional_size: u32) {
        self.size += additional_size;
        for comp in &mut self.components {
            comp.1.extend(additional_size);
        }
    }

    pub fn add_component_type<T: Component>(&mut self, storage: ComponentStorageType) {
        if self.components.insert(TypeId::of::<T>(), Box::new(ComponentStorage::<T>::new(storage, self.size))).is_some() {
            panic!("Overwriting a component type");
        }
    }
}

// todo: better name?
pub struct RuntimeBlocksBuilder {
    // todo: this
}

#[cfg(test)]
mod tests {
    use crate::block::{Component, ComponentStorageType, RuntimeBlocks};

    #[derive(Debug, Clone, PartialEq)]
    struct TestComponent {
        val: i32,
    }
    impl Component for TestComponent {}

    #[test]
    fn test() {
        let mut rids = RuntimeBlocks::new();
        rids.extend(2);
        rids.add_component_type::<TestComponent>(ComponentStorageType::Vector);

        let comp = TestComponent {
            val: 7,
        };
        rids.set_component(1, comp.clone());
        assert_eq!(rids.component::<TestComponent>(1), Some(&comp));
        assert_eq!(rids.component::<TestComponent>(0), None);
    }
}
