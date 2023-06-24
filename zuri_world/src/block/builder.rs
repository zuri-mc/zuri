use crate::block::component::{
    AnyComponentStorage, Component, ComponentStorage, ComponentStorageType,
};
use crate::block::vanilla::vanilla_block_map;
use crate::block::{
    Block, BlockMap, BlockType, BlockTypeIterator, BlockTypeIteratorInner, PropertyValue,
    RuntimeId, ToRuntimeId,
};
use std::any::TypeId;
use std::borrow::Cow;
use std::collections::btree_map::BTreeMap;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use thiserror::Error;

/// Allows for the creation of a [BlockMap] ready for use in the client.
#[derive(Clone)]
#[cfg_attr(feature = "bevy", derive(bevy::prelude::Resource))]
pub struct BlockMapBuilder {
    blocks: HashSet<BlockType>,
    /// Maps the [TypeId] of a component to a function that creates a [ComponentStorage] for it.
    components: BTreeMap<TypeId, Arc<dyn Fn(usize) -> Box<dyn AnyComponentStorage> + Sync + Send>>,
}

impl BlockMapBuilder {
    /// Initialises a [BlockMapBuilder] with all known vanilla block states.
    pub fn vanilla() -> Self {
        vanilla_block_map()
    }

    /// Create a new, empty [BlockMapBuilder].
    ///
    /// Usually you want to use [Self::vanilla] to get a builder initialised with all known block
    /// states in vanilla minecraft.
    pub fn empty() -> Self {
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
        for mut block_type in blocks {
            let variant_count = block_type.variant_count();

            for i in 0..block_type.variant_count() {
                variant_map.push((block_type.identifier.clone(), i as u32))
            }
            block_type.base_runtime_id = Some(RuntimeId(current_rid as u32));
            block_rid_map.insert(block_type, RuntimeId(current_rid as u32));

            current_rid += variant_count;
        }

        let mut components = HashMap::with_capacity(self.components.len());
        for (comp_type, storage_fn) in self.components {
            components.insert(comp_type, storage_fn(runtime_id_count));
        }

        BlockMap {
            blocks_types: block_rid_map,
            runtime_id_count: runtime_id_count as u32,
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

/// Represents a possible variant of a block type.
///
/// [BlockBuilder] is similar to [Block], but unlike the latter a BlockBuilder does not have to
/// reference an existing block variant. This makes it easier to work with and allows properties to
/// be set dynamically.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BlockBuilder<'a> {
    identifier: Cow<'a, str>,
    properties: BTreeMap<Cow<'a, str>, PropertyValue<'a>>,
}

impl<'a> BlockBuilder<'a> {
    /// Creates a new [BlockBuilder] for a block type with a certain unique identifier and an empty
    /// properties list.
    pub fn new(identifier: impl Into<Cow<'a, str>>) -> Self {
        Self {
            identifier: identifier.into(),
            properties: Default::default(),
        }
    }

    /// Gives the block's unique identifier corresponding to its block type.
    pub fn identifier(&self) -> &str {
        self.identifier.as_ref()
    }

    /// Gives a mutable reference to the unique identifier for the block type.
    ///
    /// May cause an allocation if the identifier was not an owned reference.
    pub fn identifier_mut(&mut self) -> &mut String {
        self.identifier.to_mut()
    }

    /// Returns an iterator that iterates over references to all properties present.
    pub fn properties(&self) -> impl Iterator<Item = (&str, &PropertyValue<'a>)> {
        self.properties.iter().map(|(k, v)| (k.as_ref(), v))
    }

    /// Returns an iterator that iterates over mutable references to all properties present.
    pub fn properties_mut(&mut self) -> impl Iterator<Item = (&str, &mut PropertyValue<'a>)> {
        self.properties.iter_mut().map(|(k, v)| (k.as_ref(), v))
    }

    /// Gives the value of a property with a certain name, if it exists.
    pub fn property(&self, name: &str) -> Option<&PropertyValue<'a>> {
        self.properties.get(name)
    }

    /// Gives a mutable reference to the value of a property with a certain name, if it exists.
    pub fn property_mut(&mut self, name: &str) -> Option<&mut PropertyValue<'a>> {
        self.properties.get_mut(name)
    }

    /// Inserts a new property with a certain value. Does not check if the actual block type allows
    /// a value of the given type until used.
    ///
    /// Panics if a property with the same name exists already.
    pub fn with_property(
        mut self,
        name: impl Into<Cow<'a, str>>,
        value: PropertyValue<'a>,
    ) -> Self {
        let name = name.into();
        if self.properties.contains_key(name.as_ref()) {
            panic!("Trying to overwrite property `{}`", name);
        }
        self.properties.insert(name, value);
        self
    }

    /// Inserts a new property with a certain value. Does not check if the actual block type allows
    /// a value of the given type until used.
    ///
    /// Returns the old value if a property with the same name already exists.
    pub fn insert_property(
        &mut self,
        name: impl Into<Cow<'a, str>>,
        value: PropertyValue<'a>,
    ) -> Option<PropertyValue<'a>> {
        self.properties.insert(name.into(), value)
    }

    /// Returns an iterator that will yield any block type that 'matches' the values in the
    /// [BlockBuilder].
    ///
    /// Matches are always variants of a [BlockType] with the same unique identifier as in the
    /// [BlockBuilder]. These are further narrowed down depending on the properties present in the
    /// builder.
    ///
    /// If the builder has a property that does not exist in the target block type or if
    /// the property exists but the value in the builder is not one of the allowed values, the
    /// iterator will be empty. If the property and its value do exist in the block type, then the
    /// iterator will only contain block variants with that exact value for the property.
    ///
    /// # Example
    /// The following can be used to iterate over all block states of the `minecraft:barrel` where
    /// the barrel is closed. In this example this would iterate over 6 block states: one for each
    /// direction the (closed) barrel can face.
    /// ```
    /// # use sorted_vec::SortedSet;
    /// # use zuri_world::block::{BlockBuilder, BlockMapBuilder, BlockType, PropertyValue, PropertyValues};
    /// # let block_map = BlockMapBuilder::empty()
    /// #     .with_block(
    /// #         BlockType::new("minecraft:barrel")
    /// #             .with_property("facing", PropertyValues::Strings(SortedSet::from(vec![
    /// #                     Box::from("down"),
    /// #                     Box::from("east"),
    /// #                     Box::from("north"),
    /// #                     Box::from("south"),
    /// #                     Box::from("up"),
    /// #                     Box::from("west"),
    /// #                 ])))
    /// #             .with_property("open", PropertyValues::Bool)
    /// #     ).build();
    /// #
    /// # let mut count = 0;
    /// BlockBuilder::new("minecraft:barrel")
    ///     .with_property("open", PropertyValue::Bool(false))
    ///     .matches(&block_map)
    ///     .for_each(|variant| {
    ///         println!("{variant}");
    /// #       count +=1;
    ///     });
    /// # assert_eq!(count, 6);
    /// ```
    pub fn matches<'b: 'a>(&self, block_map: &'b BlockMap) -> impl Iterator<Item = Block> {
        block_map
            .block_type(self.identifier())
            .map(|v| {
                for (name, _) in &self.properties {
                    // When the BlockBuilder has a property not present in the block type with the
                    // same ID present in the block map, the BlockBuilder matches nothing.
                    if !v.has_property(name.as_ref()) {
                        return BlockTypeIterator(BlockTypeIteratorInner::Exhausted);
                    }
                }
                v.variants()
            })
            .unwrap_or(BlockTypeIterator(BlockTypeIteratorInner::Exhausted))
            .filter(|v| {
                for (name, prop) in &self.properties {
                    // Filter any block variants that do not have the property in question or have
                    // a different value for the property.
                    if v.property_value(name.as_ref()) != Some(prop.clone()) {
                        return false;
                    }
                }
                true
            })
    }
}

impl<'a> ToRuntimeId for &'a BlockBuilder<'a> {
    type Err = BlockBuilderErr<'a>;

    fn to_runtime_id(self, block_map: &BlockMap) -> Result<RuntimeId, Self::Err> {
        let (block_type, base) = block_map
            .blocks_types
            .get_key_value(self.identifier())
            .ok_or_else(|| BlockBuilderErr::UnknownIdentifier(Cow::Borrowed(self.identifier())))?;
        let base = base.clone();
        let mut offset = 0;

        if self.properties.len() < block_type.properties.len() {
            for (name, _) in &block_type.properties {
                if self.properties.contains_key(name.as_ref()) {
                    continue;
                }
                return Err(BlockBuilderErr::<'a>::MissingProperty(
                    Cow::Borrowed(self.identifier.as_ref()),
                    Cow::Owned(name.to_string()),
                ));
            }
        }
        for (name, value) in self.properties.iter() {
            let values = block_type.property_values(&name).ok_or_else(|| {
                BlockBuilderErr::UnknownProperty(
                    Cow::Borrowed(self.identifier()),
                    Cow::Borrowed(name.as_ref()),
                )
            })?;
            let value_index = values.find_index(value).ok_or_else(|| {
                BlockBuilderErr::UnknownPropertyValue(
                    Cow::Borrowed(self.identifier()),
                    Cow::Borrowed(name.as_ref()),
                    value.to_borrowed(),
                )
            })?;

            offset = offset * values.variant_count() as u32 + value_index as u32;
        }
        Ok(RuntimeId(base.0 + offset))
    }
}

/// Returned when a [BlockBuilder] could not be converted to a single unique [RuntimeId].
///
/// All variants of the enum are structured the same, however not all variants have all fields.
///  * **Field 1:** Block identifier
///  * **Field 2:** Property name
///  * **Field 3:** Property value
#[derive(Debug, Error, Clone)]
pub enum BlockBuilderErr<'a> {
    /// The builder contains a property that the [BlockType] defined in the block map does not.
    #[error("missing property `{1}` in builder for block type `{0}`")]
    MissingProperty(Cow<'a, str>, Cow<'a, str>),
    /// The builder's unique identifier could not be found in the block map.
    #[error("block type with identifier `{0}` not found")]
    UnknownIdentifier(Cow<'a, str>),
    /// The builder defines a property that the corresponding [BlockType] does not.
    #[error("block property with name `{1}` for block `{0}` not found")]
    UnknownProperty(Cow<'a, str>, Cow<'a, str>),
    /// A property in the builder has a value that is not allowed for the corresponding property in
    /// the [BlockType].
    #[error("value `{2}` is not allowed for block property `{1}` in block type `{0}`")]
    UnknownPropertyValue(Cow<'a, str>, Cow<'a, str>, PropertyValue<'a>),
}

impl<'a> BlockBuilderErr<'a> {
    /// Clones the error, returning an error with a static lifetime.
    pub fn to_static(&self) -> BlockBuilderErr<'static> {
        match self {
            BlockBuilderErr::MissingProperty(a, b) => BlockBuilderErr::MissingProperty(
                Cow::Owned(a.to_string()),
                Cow::Owned(b.to_string()),
            ),
            BlockBuilderErr::UnknownIdentifier(a) => {
                BlockBuilderErr::UnknownIdentifier(Cow::Owned(a.to_string()))
            }
            BlockBuilderErr::UnknownProperty(a, b) => BlockBuilderErr::UnknownProperty(
                Cow::Owned(a.to_string()),
                Cow::Owned(b.to_string()),
            ),
            BlockBuilderErr::UnknownPropertyValue(a, b, c) => {
                BlockBuilderErr::UnknownPropertyValue(
                    Cow::<'static, str>::Owned(a.to_string()),
                    Cow::<'static, str>::Owned(b.to_string()),
                    match c {
                        PropertyValue::String(v) => {
                            PropertyValue::<'static>::String(Cow::<'static, str>::Owned(
                                v.to_string(),
                            ))
                        }
                        PropertyValue::Int(v) => PropertyValue::<'static>::Int(*v),
                        PropertyValue::Bool(v) => PropertyValue::<'static>::Bool(*v),
                    },
                )
            }
        }
    }
}

impl<'a> ToRuntimeId for BlockBuilder<'a> {
    type Err = BlockBuilderErr<'static>;

    fn to_runtime_id(self, block_map: &BlockMap) -> Result<RuntimeId, Self::Err> {
        match (&self).to_runtime_id(block_map) {
            Ok(v) => Ok(v),
            Err(err) => Err(err.to_static()),
        }
    }
}
