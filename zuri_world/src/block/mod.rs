use std::any::TypeId;
use std::borrow::{Borrow, Cow};
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};
use std::fmt::{Display, Formatter, Write};
use std::hash::{Hash, Hasher};
use std::io;
use std::ops::Deref;

pub use sorted_vec::SortedSet;
use thiserror::Error;

pub use builder::{BlockBuilder, BlockMapBuilder};
pub use vanilla::AIR_ID;

use crate::block::component::*;

mod builder;
pub mod component;
mod vanilla;

/// Holds all known block types and runtime blocks.
///
/// Each variant of a block type (block state) can have its own values for components.
#[derive(Debug)]
pub struct BlockMap {
    /// Maps all existing block types to their first runtime id.
    blocks_types: HashMap<BlockType, RuntimeId>,
    runtime_id_count: u32,
    variant_map: Vec<(Box<str>, u32)>,
    components: HashMap<TypeId, Box<dyn AnyComponentStorage>>,
}

impl BlockMap {
    /// Returns the amount of known runtime ids. The number returned is one higher than the largest
    /// runtime id.
    pub fn runtime_ids(&self) -> u32 {
        self.runtime_id_count
    }

    /// Get the [BlockType] for a certain unique block identifier, if it exists.
    pub fn block_type(&self, identifier: &str) -> Option<&BlockType> {
        self.blocks_types.get_key_value(identifier).map(|(k, _v)| k)
    }

    /// Gets the block variant with the provided runtime id. Corresponds with a block state.
    ///
    /// To get a block type instead, consider using [Self::block_type].
    ///
    /// Returns None if the input could not be converted to a runtime id.
    pub fn block<T: ToRuntimeId>(&self, runtime_id: T) -> Result<Block, T::Err> {
        // todo: improve

        let (block_type, variant) = self
            .variant_map
            .get(runtime_id.to_runtime_id(self)?.0 as usize)
            .unwrap();
        let block_type = self.block_type(&block_type).unwrap();
        Ok(block_type.variants().nth(*variant as usize).unwrap())
    }

    /// Get the value of component [T] for a block with the provided runtime id.
    ///
    /// Returns none even if the input could not be converted to a valid runtime id.
    pub fn component<T: Component>(&self, runtime_id: impl ToRuntimeId) -> Option<&T> {
        self.components::<T>()
            .get(runtime_id.to_runtime_id(self).ok()?)
    }

    /// Gets the component storage of a certain type, which contains all the components of that
    /// type.
    pub fn components<T: Component>(&self) -> &ComponentStorage<T> {
        self.components
            .get(&TypeId::of::<T>())
            .unwrap_or_else(|| {
                panic!(
                    "Component of type `{}` is not registered.",
                    std::any::type_name::<T>()
                )
            })
            .downcast_ref()
            .unwrap()
    }

    /// Get a mutable reference to component [T] for a block with the provided runtime id.
    ///
    /// Returns None even if the input could not be converted to a runtime id.
    pub fn component_mut<T: Component>(&mut self, runtime_id: impl ToRuntimeId) -> Option<&mut T> {
        let runtime_id = runtime_id.to_runtime_id(self);
        self.components_mut::<T>().get_mut(runtime_id.ok()?)
    }

    /// Provides a mutable reference to the [ComponentStorage] for a component type.
    pub fn components_mut<T: Component>(&mut self) -> &mut ComponentStorage<T> {
        self.components
            .get_mut(&TypeId::of::<T>())
            .unwrap_or_else(|| {
                panic!(
                    "Component of type `{}` is not registered.",
                    std::any::type_name::<T>()
                )
            })
            .downcast_mut()
            .unwrap()
    }

    /// Sets a component for a block with a certain runtime id. Overrides the previously set value
    /// if this block already had the component.
    ///
    /// Panics if the input index could not be converted to a valid runtime id.
    pub fn set_component<T: Component>(&mut self, index: impl ToRuntimeId, value: T) {
        let index = index
            .to_runtime_id(self)
            .expect("Could not convert to runtime id");
        self.components_mut::<T>().set(index, value);
    }

    /// Dumps all block states in the runtime id order. Useful for debugging.
    #[allow(dead_code)]
    pub(crate) fn dump_states(
        &self,
        writer: &mut impl io::Write,
        include_ids: bool,
    ) -> io::Result<()> {
        for runtime_id in 0..self.runtime_id_count {
            let block = self.block(runtime_id);
            if include_ids {
                writer.write(format!("{runtime_id}: ").as_bytes())?;
            }

            if let Err(err) = block {
                writer.write(format!("? ({err})\n").as_bytes())?;
                continue;
            }
            writer.write(format!("{}\n", block.unwrap()).as_bytes())?;
        }
        Ok(())
    }
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
    /// The lowest runtime id out of all the variants of this block.
    ///
    /// Gets computed when the block type gets added to a block map.
    base_runtime_id: Option<RuntimeId>,
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
            base_runtime_id: None,
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
        if self.properties.len() == 0 {
            return BlockTypeIterator(BlockTypeIteratorInner::Single(self));
        }
        BlockTypeIterator(BlockTypeIteratorInner::Multiple {
            block_type: self,
            properties: self
                .properties
                .iter()
                .map(|(_, values)| (0, values.variant_count() as u32))
                .collect(),
            next_runtime_id: self.base_runtime_id.expect(
                "Cannot iterate over variants of BlockType that has not yet been registered",
            ),
        })
    }
}

/// A set of values of a certain type that a property can have.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropertyValues {
    /// A property can have a set of string values.
    Strings(SortedSet<Box<str>>),
    /// Boolean properties can have the values `true` or `false`.
    Bool,
    /// A property can have a set of integers for its values. There can be 'holes': `1, 2, 3, 5` is
    /// allowed.
    Ints(SortedSet<i32>),
}

impl PropertyValues {
    /// Returns the amount of values present.
    pub fn variant_count(&self) -> usize {
        match self {
            PropertyValues::Strings(v) => v.len(),
            PropertyValues::Bool => 2,
            PropertyValues::Ints(v) => v.len(),
        }
    }

    /// Returns the nth value in the set. Panics if the value is not found.
    pub fn value(&self, index: usize) -> PropertyValue {
        if index >= self.variant_count() {
            panic!("Index {} out of range", index);
        }
        match self {
            PropertyValues::Strings(s) => PropertyValue::String(Cow::Borrowed(&s[index])),
            PropertyValues::Bool => PropertyValue::Bool(index == 1), // todo: check the order of this
            PropertyValues::Ints(s) => PropertyValue::Int(s[index]),
        }
    }

    fn find_index(&self, value: &PropertyValue) -> Option<usize> {
        match self {
            PropertyValues::Strings(values) => {
                if let PropertyValue::String(s) = value {
                    for (i, value) in values.iter().enumerate() {
                        if value.as_ref() != s {
                            continue;
                        }
                        return Some(i);
                    }
                }
                None
            }
            PropertyValues::Bool => match value {
                PropertyValue::Bool(true) => Some(1),
                PropertyValue::Bool(false) => Some(0),
                _ => None,
            },
            PropertyValues::Ints(values) => {
                if let PropertyValue::Int(s) = value {
                    for (i, value) in values.iter().cloned().enumerate() {
                        if value != *s {
                            continue;
                        }
                        return Some(i);
                    }
                }
                None
            }
        }
    }
}

/// An iterator that iterates over all variants in a [BlockType]. See [BlockType::variants] for
// additional info.
#[derive(Debug, Clone)]
pub struct BlockTypeIterator<'a>(BlockTypeIteratorInner<'a>);

/// The data contained by a [BlockTypeIterator]. Hidden to disallow field access.
#[derive(Debug, Clone)]
enum BlockTypeIteratorInner<'a> {
    /// The iterator contains no more block variants.
    Exhausted,
    /// The iterator has at least one remaining variant of a block with one or more properties.
    Multiple {
        /// The 'owning' [BlockType] of all the variants returned by the iterator.
        block_type: &'a BlockType,
        /// Maps properties to a value from its set of allowed values as first field and the amount of
        /// allowed values as second field. The index of the slice corresponds with the index of the
        /// property in [BlockType].
        properties: Box<[(u32, u32)]>,
        /// The runtime id of the next variant returned by the iterator.
        next_runtime_id: RuntimeId,
    },
    /// The iterator has exactly one remaining variant, which is a variant of a [BlockType] without
    /// any properties.
    Single(&'a BlockType),
}

impl<'a> Iterator for BlockTypeIterator<'a> {
    type Item = Block<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            BlockTypeIteratorInner::Exhausted => None,
            BlockTypeIteratorInner::Multiple {
                block_type,
                properties,
                next_runtime_id,
            } => {
                let mut exhaust = false;
                let next = properties
                    .iter()
                    .cloned()
                    .map(|(value, _value_count)| value)
                    .collect();
                for (i, (value, value_count)) in properties.iter_mut().enumerate().rev() {
                    *value += 1;
                    if value < value_count {
                        break;
                    }
                    if i == 0 {
                        exhaust = true;
                    }
                    *value = 0
                }

                let ret = Some(Block {
                    block_type: *block_type,
                    properties: next,
                    runtime_id: next_runtime_id.clone(),
                });
                next_runtime_id.0 += 1;
                if exhaust {
                    self.0 = BlockTypeIteratorInner::Exhausted;
                }
                ret
            }
            BlockTypeIteratorInner::Single(block_type) => {
                let ret = Some(Block {
                    block_type,
                    properties: Box::new([]),
                    runtime_id: block_type.base_runtime_id.unwrap(),
                });
                self.0 = BlockTypeIteratorInner::Exhausted;
                ret
            }
        }
    }
}

/// Block is a reference to a variant of a certain [BlockType].
///
/// Values of type [Block] are guaranteed to reference an existing block variant. This can make it
/// harder to use. [BlockBuilder] acts as an easier to use equivalent of this type, without having
/// the guarantee that it defines a known block.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Block<'a> {
    block_type: &'a BlockType,
    /// Maps properties to a value from its set of allowed values. The index of the slice
    /// corresponds with the index of the property in [BlockType].
    properties: Box<[u32]>,
    runtime_id: RuntimeId,
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

    /// The runtime id of the block. See [RuntimeId].
    pub fn runtime_id(&self) -> RuntimeId {
        self.runtime_id
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

impl<'a> Into<RuntimeId> for Block<'a> {
    fn into(self) -> RuntimeId {
        self.runtime_id
    }
}

impl<'a> Into<RuntimeId> for &Block<'a> {
    fn into(self) -> RuntimeId {
        self.runtime_id
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
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum PropertyValue<'a> {
    String(Cow<'a, str>),
    Bool(bool),
    Int(i32),
}

impl<'a> PropertyValue<'a> {
    /// Ensures the [PropertyValue] does not own any values. (In this case, the string value is
    /// borrowed)
    pub fn to_borrowed(&'a self) -> PropertyValue<'a> {
        match self {
            PropertyValue::String(v) => PropertyValue::String(Cow::Borrowed(v.as_ref())),
            v => v.clone(),
        }
    }
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

/// A block runtime id.
///
/// Each combination of a unique block identifier and a set of properties mapped to one of the
/// possible property values has its own runtime id. The amount of unique runtime ids each
/// [BlockType] has corresponds with [BlockType::variant_count]. Runtime ids are also unique between
/// different block types.
#[derive(Debug, Copy, Clone, Eq, Ord, Hash)]
pub struct RuntimeId(pub u32);

/// Allows for types to be converted to a [RuntimeId] given a [BlockType].
pub trait ToRuntimeId {
    /// The error to return when conversion to a runtime id has failed.
    type Err: std::error::Error;

    /// Get the [RuntimeId] corresponding with the value of the type.
    fn to_runtime_id(self, block_map: &BlockMap) -> Result<RuntimeId, Self::Err>;
}

/// Automatically implement [ToRuntimeId] when conversion is trivial and doesn't need [BlockMap].
impl<T: Into<RuntimeId>> ToRuntimeId for T {
    type Err = OutOfRangeError;

    fn to_runtime_id(self, block_map: &BlockMap) -> Result<RuntimeId, Self::Err> {
        let rid = self.into();
        if rid >= block_map.runtime_ids() {
            return Err(OutOfRangeError);
        }
        Ok(rid)
    }
}

/// Returned when conversion to a runtime id results in a runtime id higher than the highest known
/// one.
#[derive(Debug, Error, Copy, Clone)]
#[error("the runtime id is out of range")]
pub struct OutOfRangeError;

impl<T: Copy + Into<RuntimeId>> PartialEq<T> for RuntimeId {
    fn eq(&self, other: &T) -> bool {
        self.0 == other.clone().into().0
    }
}

impl<T: Copy + Into<RuntimeId>> PartialOrd<T> for RuntimeId {
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        self.0.partial_cmp(&other.clone().into().0)
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
