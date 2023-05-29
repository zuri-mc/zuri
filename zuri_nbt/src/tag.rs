//! Contains all NBT tags.
use crate::NBTTag;
use std::collections::HashMap;

/// An 8-bit unsigned integer.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct Byte(pub u8);

/// A 16-bit signed integer.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct Short(pub i16);

/// A 32-bit signed integer.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct Int(pub i32);

/// A 64-bit signed integer.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct Long(pub i64);

/// A 32-bit floating point number.
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Float(pub f32);

/// A 64-bit floating point number.
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Double(pub f64);

/// A string of characters.
///
/// Should never be larger than [i16::MAX].
#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct String(pub std::string::String);

/// A map containing zero or more key-value pairs.
///
/// Each key maps to exactly one [NBTTag] of any type.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Compound(pub HashMap<std::string::String, NBTTag>);

/// A variable-length list [NBTTag]s of the same type.
///
/// Lists will fail to encode/decode should it contain values of which the type does not match
/// the type of the first element in the list.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct List(pub Vec<NBTTag>);

/// A variable-length array containing 8-bit unsigned integers.
#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct ByteArray(pub Vec<u8>);

/// A variable-length array containing 32-bit signed integers.
#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct IntArray(pub Vec<i32>);

/// A variable-length array containing 64-bit signed integers.
#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct LongArray(pub Vec<i64>);

/// Contains utilities for the [Compound] NBT tag.
pub mod compound {
    use crate::NBTTag;

    /// Allows for a more ergonomic way of creating NBT compound tags.
    #[must_use]
    #[derive(Debug, Default)]
    pub struct Builder {
        value: super::Compound,
    }

    impl Builder {
        /// Consume the builder and return the underlying compound tag.
        #[must_use]
        pub fn build(self) -> super::Compound {
            self.value
        }

        /// Inserts a new NBT tag into the underlying compound tag under the provided key.
        ///
        /// Panics when inserting with a key that already exists.
        pub fn with<T: Into<NBTTag>>(mut self, key: impl Into<String>, value: T) -> Self {
            let key = key.into();
            if let Some(val) = self.value.0.get(&key) {
                panic!("trying to overwrite key `{key}` that has value: {val:?}",);
            }
            self.value.0.insert(key.into(), value.into());
            self
        }
    }

    impl super::Compound {
        /// Returns a new builder object to create a compound tag.
        pub fn builder() -> Builder {
            Builder {
                value: Default::default(),
            }
        }
    }
}
