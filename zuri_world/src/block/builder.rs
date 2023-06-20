use crate::block::component::{
    AnyComponentStorage, Component, ComponentStorage, ComponentStorageType,
};
use crate::block::{BlockMap, BlockType, RuntimeId};
use fnv::FnvHasher;
use std::any::TypeId;
use std::collections::btree_map::BTreeMap;
use std::collections::{HashMap, HashSet};
use std::hash::BuildHasher;
use std::sync::Arc;

/// Allows for the creation of a [BlockMap] ready for use in the client.
#[derive(Clone)]
pub struct BlockMapBuilder {
    blocks: HashSet<BlockType, FnvHashBuilder>,
    /// Maps the [TypeId] of a component to a function that creates a [ComponentStorage] for it.
    components: BTreeMap<TypeId, Arc<dyn Fn(usize) -> Box<dyn AnyComponentStorage>>>,
}

impl BlockMapBuilder {
    /// Create a new, empty [BlockMapBuilder].
    pub fn new() -> Self {
        Self {
            blocks: Default::default(),
            components: Default::default(),
        }
    }

    /// Add a new block type to the builder.
    ///
    /// Panics if the builder already has a block with the same unique identifier as the new block.
    pub fn with_block(mut self, b: BlockType) -> Self {
        if self.blocks.contains(b.identifier.as_ref()) {
            panic!(
                "Identifier `{}` is already present in builder",
                b.identifier.as_ref()
            );
        }
        self.blocks.insert(b);
        self
    }

    /// Insert a new block to the builder, replacing the old block with the same unique identifier
    /// if present.
    pub fn insert_block(&mut self, b: BlockType) -> Option<BlockType> {
        self.blocks.replace(b)
    }

    /// See [Self::insert_component_type].
    pub fn with_component_type<T: Component>(mut self, storage: ComponentStorageType) -> Self {
        self.insert_component_type::<T>(storage);
        self
    }

    /// Register a new component type along with the storage type it should use.
    ///
    /// If you are uncertain about what storage type to choose, use
    /// [ComponentStorageType::default()].
    ///
    /// Panics if a component of the same type is already added.
    pub fn insert_component_type<T: Component>(&mut self, storage: ComponentStorageType) {
        if self
            .components
            .insert(
                TypeId::of::<T>(),
                Arc::new(move |cap| Box::new(ComponentStorage::<T>::new(storage, cap as u32))),
            )
            .is_some()
        {
            panic!(
                "Overwriting component type `{:?}`",
                std::any::type_name::<T>()
            );
        }
    }

    /// Create a [BlockMap] from the data in the builder, consuming it in the process.
    pub fn build(mut self) -> BlockMap {
        self.blocks.shrink_to_fit();

        let mut runtime_id_count = 0;
        let mut block_rid_map =
            HashMap::with_capacity_and_hasher(self.blocks.len(), FnvHashBuilder);
        for block_type in self.blocks {
            let variant_count = block_type.variant_count();
            block_rid_map.insert(block_type, RuntimeId(runtime_id_count as u32));
            runtime_id_count += variant_count;
        }

        let mut components = HashMap::with_capacity(self.components.len());
        for (comp_type, storage_fn) in self.components {
            components.insert(comp_type, storage_fn(runtime_id_count));
        }

        BlockMap {
            blocks_types: block_rid_map,
            _runtime_id_count: runtime_id_count as u32,
            components,
        }
    }
}

#[derive(Default, Copy, Clone)]
pub(super) struct FnvHashBuilder;

impl BuildHasher for FnvHashBuilder {
    type Hasher = FnvHasher;

    fn build_hasher(&self) -> Self::Hasher {
        FnvHasher::with_key(0x811c9dc5)
    }
}
