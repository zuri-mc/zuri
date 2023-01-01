pub mod geometry;

/// The base trait for any type of component. A component does not actually require to have any
/// special methods, besides that it should implement the [`std::fmt::Debug`] trait. A component
/// should also be [`Send`] + [`Sync`].
/// Components should ideally not be modified after being inserted.
pub trait Component: std::fmt::Debug + downcast_rs::DowncastSync {}
downcast_rs::impl_downcast!(sync Component);

/// The different ways component of a same type can be stored. Each has different advantages and
/// drawbacks.
pub enum ComponentStorageType {
    // todo: document each type
    Vector,
    Compact,
    Hashed,
}

/// Stores components for all RIDs. Can use a few different types of storage methods (see
/// [`ComponentStorageType`]).
#[derive(Debug)]
pub struct ComponentStorage<T: Component> {
    storage: ComponentStorageImpl<T>,
}

impl<T: Component> ComponentStorage<T> {
    pub(super) fn new(storage: ComponentStorageType, size: u32) -> Self {
        Self {
            storage: match storage {
                ComponentStorageType::Vector => ComponentStorageImpl::Vector((0..size).map(|_| None).collect()),
                ComponentStorageType::Compact => todo!(),
                ComponentStorageType::Hashed => todo!(),
            },
        }
    }

    pub fn get(&self, rid: u32) -> Option<&T> {
        match &self.storage {
            ComponentStorageImpl::Vector(v) => match v.get(rid as usize).unwrap() {
                None => None,
                Some(v) => Some(v),
            },
            ComponentStorageImpl::Compact() => todo!(),
            ComponentStorageImpl::Hashed() => todo!(),
        }
    }

    pub fn set(&mut self, index: u32, comp: T) {
        match &mut self.storage {
            ComponentStorageImpl::Vector(v) => v[index as usize] = Some(comp),
            ComponentStorageImpl::Compact() => todo!(),
            ComponentStorageImpl::Hashed() => todo!(),
        };
    }
}

// Internal
// --------

#[derive(Debug)]
pub(super) enum ComponentStorageImpl<T: Component> {
    Vector(Vec<Option<T>>),
    Compact(/* todo */),
    Hashed(/* todo */),
}

pub(super) trait IComponentStorage: downcast_rs::DowncastSync {
    fn extend(&mut self, additional_size: u32);
}
downcast_rs::impl_downcast!(sync IComponentStorage);

impl<T: Component> IComponentStorage for ComponentStorage<T> {
    fn extend(&mut self, additional_size: u32) {
        match &mut self.storage {
            ComponentStorageImpl::Vector(v) => v.resize_with(
                v.len() + additional_size as usize,
                || None,
            ),
            ComponentStorageImpl::Compact() => todo!(),
            ComponentStorageImpl::Hashed() => todo!(),
        };
    }
}
