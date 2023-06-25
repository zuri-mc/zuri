use crate::block::BlockMap;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use thiserror::Error;

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
