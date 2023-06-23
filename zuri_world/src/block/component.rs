use crate::block::RuntimeId;
use std::collections::HashMap;
use std::fmt::Debug;

pub mod geometry;

/// The base trait for any type of component.
///
/// A component does not actually require to have any special methods, besides that it should
/// implement the [Debug] trait. A component should also be [Send] + [Sync].
///
/// Components should ideally not be modified after being inserted.
pub trait Component: Debug + Send + Sync + 'static {}

/// The different ways component of a same type can be stored. Each has different advantages and
/// drawbacks.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub enum ComponentStorageType {
    /// Stores components in a vector with a capacity equal to the amount of runtime ids.
    ///
    /// Useful for when a lot of blocks are expected to have the component.
    ///
    /// **Characteristics**
    ///  - O(1) random access
    ///  - Iterating is fast (provided a lot of blocks have the component)
    ///  - Larger memory overhead
    #[default]
    Vector,
    /// Stores components in an ordered vector which, unlike [Self::Vector], only uses space for
    /// blocks that have the component.
    ///
    /// **Characteristics**
    ///  - O(log(n)) random access
    ///  - Iterating is fast
    Compact,
    /// Stores components in a [HashMap] using [ahash].
    ///
    /// **Characteristics**
    ///  - O(1) random access
    ///  - Iterating is slower
    Hashed,
}

/// Stores components for all RIDs. Can use a few different types of storage methods (see
/// [`ComponentStorageType`]).
#[derive(Debug)]
pub struct ComponentStorage<T: Component> {
    storage: ComponentStorageImpl<T>,
}

impl<T: Component> ComponentStorage<T> {
    /// Creates a new [ComponentStorage] using a certain type of component storage.
    ///
    /// Expects a maximum expected capacity that corresponds to the amount of runtime ids. Note that
    /// providing a capacity does not guarantee that the underlying storage actually reserves this
    /// amount of data depending on the storage type. This is because some storage types are
    /// specifically for components that are not implemented by a lot of runtime ids.
    pub(super) fn new(storage: ComponentStorageType, capacity: u32) -> Self {
        Self {
            storage: match storage {
                ComponentStorageType::Vector => {
                    ComponentStorageImpl::Vector((0..capacity).map(|_| None).collect())
                }
                ComponentStorageType::Compact => ComponentStorageImpl::Compact(Default::default()),
                ComponentStorageType::Hashed => ComponentStorageImpl::Hashed(Default::default()),
            },
        }
    }

    /// Returns a reference to the component's value corresponding to the provided runtime id, if it
    /// exists.
    pub fn get(&self, rid: impl Into<RuntimeId>) -> Option<&T> {
        match &self.storage {
            ComponentStorageImpl::Vector(v) => match v.get(rid.into().0 as usize).unwrap() {
                None => None,
                Some(v) => Some(v),
            },
            ComponentStorageImpl::Compact(v) => {
                let new_runtime_id = rid.into();

                let index = v
                    .binary_search_by(|(elem, _)| elem.cmp(&new_runtime_id))
                    .ok();
                index.map(|index| v.get(index).map(|(_, v)| v)).flatten()
            }
            ComponentStorageImpl::Hashed(v) => v.get(&rid.into()),
        }
    }

    /// Returns mutable a reference to the component's value corresponding to the provided runtime
    /// id, if it exists.
    pub fn get_mut(&mut self, rid: impl Into<RuntimeId>) -> Option<&mut T> {
        match &mut self.storage {
            ComponentStorageImpl::Vector(v) => match v.get_mut(rid.into().0 as usize).unwrap() {
                None => None,
                Some(v) => Some(v),
            },
            ComponentStorageImpl::Compact(v) => {
                let new_runtime_id = rid.into();

                let index = v
                    .binary_search_by(|(elem, _)| elem.cmp(&new_runtime_id))
                    .ok();
                index
                    .map(|index| v.get_mut(index).map(|(_, v)| v))
                    .flatten()
            }
            ComponentStorageImpl::Hashed(v) => v.get_mut(&rid.into()),
        }
    }

    /// Sets a component, inserting a new value or overriding an already existing value.
    pub fn set(&mut self, index: impl Into<RuntimeId>, comp: T) {
        match &mut self.storage {
            ComponentStorageImpl::Vector(v) => v[index.into().0 as usize] = Some(comp),
            ComponentStorageImpl::Compact(v) => {
                let new_runtime_id = index.into();

                let index = v
                    .binary_search_by(|(elem, _)| elem.cmp(&new_runtime_id))
                    .unwrap_or_else(|e| e);

                v.insert(index, (new_runtime_id, comp));
            }
            ComponentStorageImpl::Hashed(v) => {
                v.insert(index.into(), comp);
            }
        };
    }
}

// Internal
// --------

#[derive(Debug)]
pub(super) enum ComponentStorageImpl<T: Component> {
    Vector(Vec<Option<T>>),
    Compact(Vec<(RuntimeId, T)>),
    Hashed(HashMap<RuntimeId, T, ahash::RandomState>),
}

pub(super) trait AnyComponentStorage: downcast_rs::DowncastSync + Debug {}
downcast_rs::impl_downcast!(sync AnyComponentStorage);

impl<T: Component> AnyComponentStorage for ComponentStorage<T> {}
