use crate::block::component::{
    AnyComponentStorage, Component, ComponentStorage, ComponentStorageType,
};
use crate::block::{BlockMap, BlockType, RuntimeId};
use std::any::TypeId;
use std::collections::btree_map::BTreeMap;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

/// Allows for the creation of a [BlockMap] ready for use in the client.
#[derive(Clone)]
pub struct BlockMapBuilder {
    blocks: HashSet<BlockType>,
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

        let runtime_id_count = self.blocks.iter().map(|v| v.variant_count()).sum();

        let mut variant_map = Vec::with_capacity(runtime_id_count);
        let mut block_rid_map = HashMap::with_capacity(self.blocks.len());

        let mut blocks = Vec::with_capacity(self.blocks.len());
        for block in self.blocks {
            blocks.push(block);
        }
        blocks.sort_by(|a, b| {
            let a_hash = hash_identifier(a.identifier.as_ref());
            let b_hash = hash_identifier(b.identifier.as_ref());

            a_hash.cmp(&b_hash)
        });

        let mut current_rid = 0;
        for block_type in blocks {
            let variant_count = block_type.variant_count();

            for i in 0..block_type.variant_count() {
                variant_map.push((block_type.identifier.clone(), i as u32))
            }
            block_rid_map.insert(block_type, RuntimeId(current_rid as u32));

            current_rid += variant_count;
        }

        let mut components = HashMap::with_capacity(self.components.len());
        for (comp_type, storage_fn) in self.components {
            components.insert(comp_type, storage_fn(runtime_id_count));
        }

        BlockMap {
            blocks_types: block_rid_map,
            _runtime_id_count: runtime_id_count as u32,
            variant_map,
            components,
        }
    }
}

/// Hashes a string using the `fnv1` hashing algorithm.
///
/// [Source](https://en.wikipedia.org/wiki/Fowler%E2%80%93Noll%E2%80%93Vo_hash_function#FNV-1_hash)
fn hash_identifier(id: &str) -> u64 {
    let mut hash = 14695981039346656037_u64;
    for byte in id.as_bytes() {
        hash = hash.wrapping_mul(1099511628211_u64);
        hash = hash ^ (*byte as u64);
    }
    hash
}
