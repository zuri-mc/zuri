use bevy::app::AppLabel;
use std::any::TypeId;
use std::borrow::Borrow;
use std::collections::{BTreeMap, HashMap};
use std::fmt::{Display, Formatter, Write};
use std::hash::{Hash, Hasher};
use std::ops::{Deref, Index};

use bevy::prelude::{Mesh, Resource};
use bevy::render::mesh::PrimitiveTopology;
use bevy_render::mesh::Indices;
use bevy_render::render_resource::encase::private::RuntimeSizedArray;
use json::JsonValue;

pub use builder::BlockMapBuilder;

use crate::block::builder::FnvHashBuilder;
use crate::block::component::geometry::Geometry;
use crate::block::component::*;

mod builder;
pub mod component;
pub mod types;

// todo: remove this temporary function. we want to eventually build all the runtime IDs from data
pub fn build_rids() -> BlockMap {
    const VANILLA_BLOCKS: &str = include_str!("vanilla_blocks.json");
    let vanilla_blocks = json::parse(VANILLA_BLOCKS).unwrap();

    let mut block_map =
        BlockMapBuilder::new().with_component_type::<Geometry>(ComponentStorageType::Vector);

    // todo: greatly improve this
    for block in vanilla_blocks["data_items"].members() {
        let mut block_type = BlockType::new(block["name"].as_str().unwrap());

        'outer: for prop in block["properties"].members() {
            for prop_definition in vanilla_blocks["block_properties"].members() {
                let name = prop["name"].as_str().unwrap();
                if Some(name) != prop_definition["name"].as_str() {
                    continue;
                }
                let values = match prop_definition["type"].as_str().unwrap() {
                    "bool" => PropertyValues::Boolean,
                    "int" => PropertyValues::Ints(
                        prop_definition["values"]
                            .members()
                            .map(|v| v["value"].as_i32().unwrap())
                            .collect(),
                    ),
                    "string" => PropertyValues::Strings(
                        prop_definition["values"]
                            .members()
                            .map(|v| Box::from(v["value"].as_str().unwrap()))
                            .collect(),
                    ),
                    _ => panic!("unknown property type"),
                };

                block_type.insert_property(name, values);
                continue 'outer;
            }
            panic!("unknown property");
        }

        if block_map.insert_block(block_type).is_some() {
            panic!("overwriting");
        }
    }

    let mut block_map = block_map.build();

    block_map.set_component(
        block_map.blocks_types.get("minecraft:air").unwrap().0,
        Geometry {
            mesh: Mesh::new(PrimitiveTopology::TriangleList),
        },
    );
    {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        let vertices: Vec<[f32; 3]> = vec![[0., 0., 0.], [0., 1., 0.], [1., 0., 1.], [1., 1., 1.]];
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        let normals: Vec<[f32; 3]> = vec![[1., 0., 0.], [1., 0., 0.], [1., 0., 0.], [1., 0., 0.]];
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        let uvs: Vec<[f32; 2]> = vec![[0., 0.], [0., 1.], [1., 0.], [1., 1.]];
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.set_indices(Some(Indices::U32(vec![0, 1, 2, 1, 2, 3, 2, 1, 0, 3, 2, 1])));

        block_map.set_component(
            block_map.blocks_types.get("minecraft:tallgrass").unwrap().0,
            Geometry { mesh },
        );
    }

    block_map
}

/// A type of minecraft block with a unique namespaced identifier.
///
/// `minecraft:dirt` and `minecraft:planks` are both examples of unique block types. Note that
/// planks for all wood types have the same unique BlockType, so spruce planks and oak planks do not
/// count as unique from each other.
///
/// Block types define a set of properties, with each property having a finite set of values. Each
/// instance of the block has one of these values for each property.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BlockType {
    /// The unique identifier of the BlockType. This is a boxed [str] as this is not meant to be
    /// modified after creation.
    identifier: Box<str>,
    /// Maps property names to a set of possible values the property can have.
    properties: BTreeMap<Box<str>, PropertyValues>,
}

/// Hashing a [BlockType] is the same as hashing its unique identifier string.
impl Hash for BlockType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.identifier.hash(state)
    }
}

/// Borrows the [BlockType]'s unique identifier.
impl Borrow<str> for BlockType {
    fn borrow(&self) -> &str {
        self.identifier.as_ref()
    }
}

impl BlockType {
    /// Creates a new [BlockType] with a unique identifier and an empty set of properties.
    pub fn new(name: impl Into<Box<str>>) -> Self {
        Self {
            identifier: name.into(),
            properties: Default::default(),
        }
    }

    /// Returns the unique namespaced identifier of the block type.
    pub fn identifier(&self) -> &str {
        self.identifier.as_ref()
    }

    /// The total amount of variants this block type has.
    ///
    /// If the block has two properties, one with 5 values and another with 2, then the total amount
    /// of variants is 10. Blocks with no properties have exactly one variant.
    pub fn variant_count(&self) -> usize {
        let mut count = 1;
        for (_, prop) in &self.properties {
            count *= prop.variant_count();
        }
        count
    }

    /// Returns true if the [BlockType] has a property with the provided name, false otherwise.
    pub fn has_property(&self, name: &str) -> bool {
        self.properties.contains_key(name)
    }

    /// Returns the set of possible values a certain property can have. Returns None if such a
    /// property does not exist.
    pub fn property_values(&self, name: &str) -> Option<&PropertyValues> {
        self.properties.get(name)
    }

    /// Adds a new property to the [BlockType].
    ///
    /// Accepts a property name, such as `direction`, as well as a set of values that the property
    /// can have.
    ///
    /// If a property with the same name already exists, this will panic.
    pub fn with_property(mut self, name: impl Into<Box<str>>, values: PropertyValues) -> Self {
        let name = name.into();
        if self.properties.contains_key(name.as_ref()) {
            panic!(
                "Trying to overwrite property `{}` using `with_property`",
                name.as_ref()
            );
        }
        self.insert_property(name, values);
        self
    }

    /// Inserts a new property to the [BlockType].
    ///
    /// Accepts a property name, such as `direction`, as well as a set of values that the property
    /// can have.
    ///
    /// If a property with the same name already exists, it will be overwritten and the old value
    /// will be returned. If this is not the case, None is returned.
    pub fn insert_property(
        &mut self,
        name: impl Into<Box<str>>,
        values: PropertyValues,
    ) -> Option<PropertyValues> {
        self.properties.insert(name.into(), values)
    }

    /// Returns an iterator that iterators over all the variants for the [BlockType].
    ///
    /// It does so by iterating over the cartesian product of all property values:
    /// ```_
    /// (A1, B1)
    /// (A1, B2)
    /// ...
    /// (A2, Bm)
    /// (A2, B1)
    /// (A2, B2)
    /// ...
    /// (An, Bm)
    /// ```
    pub fn variants(&self) -> BlockTypeIterator {
        BlockTypeIterator {
            block_type: self,
            properties: self
                .properties
                .iter()
                .map(|(_, values)| (0, values.variant_count() as u32))
                .collect(),
            exhausted: false,
        }
    }
}

/// A set of values of a certain type that a property can have.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropertyValues {
    Strings(Box<[Box<str>]>),
    // todo: better public-facing types
    Boolean,
    Ints(Box<[i32]>),
}

impl PropertyValues {
    /// Returns the amount of values present.
    pub fn variant_count(&self) -> usize {
        match self {
            PropertyValues::Strings(v) => v.len(),
            PropertyValues::Boolean => 2,
            PropertyValues::Ints(v) => v.len(),
        }
    }

    /// Returns the nth value in the set. Panics if the value is not found.
    pub fn value(&self, index: usize) -> PropertyValue {
        if index >= self.variant_count() {
            panic!("Index {} out of range", index);
        }
        match self {
            PropertyValues::Strings(s) => PropertyValue::String(&s[index]),
            PropertyValues::Boolean => PropertyValue::Bool(index == 1), // todo: check the order of this
            PropertyValues::Ints(s) => PropertyValue::Int(s[index]),
        }
    }
}

/// An iterator that iterates over all variants in a [BlockType]. See [BlockType::variants] for
// additional info.
#[derive(Debug, Clone)]
pub struct BlockTypeIterator<'a> {
    block_type: &'a BlockType,
    /// Maps properties to a value from its set of allowed values as first field and the amount of
    /// allowed values as second field. The index of the slice corresponds with the index of the
    /// property in [BlockType].
    properties: Box<[(u32, u32)]>,
    /// Used to determine if the iterator has been exhausted **when the [BlockType] has no
    /// properties**.
    exhausted: bool,
}

impl<'a> Iterator for BlockTypeIterator<'a> {
    type Item = Block<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.properties.len() == 0 {
            if self.exhausted {
                return None;
            }
            self.exhausted = true;
            return Some(Block {
                block_type: self.block_type,
                properties: Default::default(),
            });
        }

        let next = self
            .properties
            .iter()
            .cloned()
            .map(|(value, value_count)| value)
            .collect();
        for (i, (value, value_count)) in self.properties.iter_mut().enumerate().rev() {
            *value += 1;
            if value < value_count {
                break;
            }
            if i == 0 {
                return None;
            }
            *value = 0
        }

        Some(Block {
            block_type: self.block_type,
            properties: next,
        })
    }
}

/// Block represents an variant of a certain [BlockType].
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Block<'a> {
    block_type: &'a BlockType,
    /// Maps properties to a value from its set of allowed values. The index of the slice
    /// corresponds with the index of the property in [BlockType].
    properties: Box<[u32]>,
}

impl<'a> Block<'a> {
    /// Returns the unique identifier of the block's [BlockType].
    pub fn identifier(&self) -> &'a str {
        self.block_type.identifier()
    }

    /// Gets the block's [BlockType].
    pub fn block_type(&self) -> &'a BlockType {
        self.block_type
    }

    /// Looks up the value of a certain property if the block has the property.
    ///
    /// Blocks always have a property value for all properties defined in it's [BlockType].
    ///
    /// As a compromise with minimizing heap usage, this method is O(n) where n is the amount of
    /// properties the block has.
    pub fn property_value(&self, property_name: &str) -> Option<PropertyValue<'a>> {
        let mut props = self.block_type.properties.iter().enumerate();
        while let Some((i, (name, values))) = props.next() {
            if (*name).deref() != property_name {
                continue;
            }
            return Some(values.value(self.properties[i] as usize));
        }
        None
    }

    /// Returns an iterator with all the block's properties.
    pub fn properties(&'a self) -> impl Iterator<Item = (&'a str, PropertyValue<'a>)> {
        self.block_type
            .properties
            .iter()
            .zip(self.properties.iter())
            .map(|((name, values), &value)| (name.as_ref(), values.value(value as usize)))
    }
}

impl<'a> ToRuntimeId for Block<'a> {
    fn to_runtime_id(self, block_map: &BlockMap) -> RuntimeId {
        let base = block_map
            .blocks_types
            .get(self.block_type.identifier())
            .unwrap()
            .0;
        let mut offset = 0;

        for ((_, values), value) in self
            .block_type
            .properties
            .iter()
            .zip(self.properties.iter())
        {
            offset = offset * values.variant_count() as u32 + *value;
        }
        RuntimeId(base + offset)
    }
}

impl<'a> Display for Block<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.block_type.identifier.fmt(f)?;

        f.write_char('{')?;
        let mut props = self.properties();
        if let Some((name, value)) = props.next() {
            name.fmt(f)?;
            f.write_char('=')?;
            value.fmt(f)?;
        }

        for (name, value) in props {
            f.write_char(',')?;
            name.fmt(f)?;
            f.write_char('=')?;
            value.fmt(f)?;
        }
        f.write_char('}')?;
        Ok(())
    }
}

/// A single, non-owned value for a property.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PropertyValue<'a> {
    String(&'a str),
    Bool(bool),
    Int(i32),
}

impl<'a> Display for PropertyValue<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PropertyValue::String(v) => {
                f.write_char('"')?;
                v.fmt(f)?;
                f.write_char('"')
            }
            PropertyValue::Bool(v) => v.fmt(f),
            PropertyValue::Int(v) => v.fmt(f),
        }
    }
}

/// Holds all known block types and runtime blocks.
#[derive(Resource, Debug)]
pub struct BlockMap {
    /// Maps all existing block types to their first runtime id.
    blocks_types: HashMap<BlockType, RuntimeId, FnvHashBuilder>,
    _runtime_id_count: u32,
    components: HashMap<TypeId, Box<dyn AnyComponentStorage>>,
}

impl BlockMap {
    /// Get the [BlockType] for a certain unique block identifier, if it exists.
    pub fn block_type(&self, identifier: &str) -> Option<&BlockType> {
        self.blocks_types.get_key_value(identifier).map(|(k, _v)| k)
    }

    /// Get the value of component [T] for a block with the provided runtime id.
    pub fn component<T: Component>(&self, runtime_id: impl ToRuntimeId) -> Option<&T> {
        self.components::<T>().get(runtime_id.to_runtime_id(self))
    }

    pub fn components<T: Component>(&self) -> &ComponentStorage<T> {
        self.components
            .get(&TypeId::of::<T>())
            .expect("Component not registered")
            .downcast_ref()
            .unwrap()
    }

    // todo: remove
    pub fn components_mut<T: Component>(&mut self) -> &mut ComponentStorage<T> {
        self.components
            .get_mut(&TypeId::of::<T>())
            .expect("Component not registered")
            .downcast_mut()
            .unwrap()
    }

    // todo: remove
    pub fn set_component<T: Component>(&mut self, index: impl ToRuntimeId, value: T) {
        let index = index.to_runtime_id(self);
        self.components_mut::<T>().set(index, value);
    }
}

/// A block runtime id.
///
/// Each combination of a unique block identifier and a set of properties mapped to one of the
/// possible property values has its own runtime id. The amount of unique runtime ids each
/// [BlockType] has corresponds with [BlockType::variant_count]. Runtime ids are also unique between
/// different block types.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RuntimeId(u32);

/// Allows for types to be converted to a [RuntimeId] given a [BlockType].
pub trait ToRuntimeId {
    /// Get the [RuntimeId] corresponding with the value of the type. // todo: Option<RuntimeId>?
    fn to_runtime_id(self, block_map: &BlockMap) -> RuntimeId;
}

/// Automatically implement [ToRuntimeId] when conversion is trivial and doesn't need [BlockMap].
impl<T: Into<RuntimeId>> ToRuntimeId for T {
    fn to_runtime_id(self, _block_map: &BlockMap) -> RuntimeId {
        self.into()
    }
}

impl Display for RuntimeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <u32 as Display>::fmt(&self.0, f)
    }
}

impl From<u32> for RuntimeId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<RuntimeId> for u32 {
    fn from(value: RuntimeId) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use crate::block::{BlockMap, Component, ComponentStorageType};

    #[derive(Debug, Clone, PartialEq)]
    struct TestComponent {
        val: i32,
    }

    impl Component for TestComponent {}

    #[test]
    fn test() {
        // todo: rewrite test
    }
}
