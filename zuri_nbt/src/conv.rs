//! Implementations for type conversions from and to [NBTTag] using [From] and [TryFrom].
use crate::{tag, NBTTag};
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

macro_rules! impl_enum_conv {
    ($typ:ty, $enum_variant:path) => {
        impl TryFrom<NBTTag> for $typ {
            type Error = NBTTag;

            fn try_from(value: NBTTag) -> Result<Self, Self::Error> {
                if let $enum_variant(v) = value {
                    Ok(v.into())
                } else {
                    Err(value)
                }
            }
        }

        impl From<$typ> for NBTTag {
            fn from(value: $typ) -> Self {
                $enum_variant(value.into())
            }
        }
    };
    ($(($typ:ty, $enum_variant:path)$(,)?)*) => {
        $(impl_enum_conv!($typ, $enum_variant);)*
    };
}

impl_enum_conv!(
    (tag::Byte, NBTTag::Byte),
    (tag::Short, NBTTag::Short),
    (tag::Int, NBTTag::Int),
    (tag::Long, NBTTag::Long),
    (tag::Float, NBTTag::Float),
    (tag::Double, NBTTag::Double),
    (tag::String, NBTTag::String),
    (tag::Compound, NBTTag::Compound),
    (tag::List, NBTTag::List),
    (tag::ByteArray, NBTTag::ByteArray),
    (tag::IntArray, NBTTag::IntArray),
    (tag::LongArray, NBTTag::LongArray),
);

macro_rules! impl_newtype_conv {
    ($typ:ty, $newtyp:path) => {
        impl From<$newtyp> for $typ {
            fn from(value: $newtyp) -> Self {
                value.0
            }
        }

        impl From<$typ> for $newtyp {
            fn from(value: $typ) -> Self {
                $newtyp(value)
            }
        }

        impl Deref for $newtyp {
            type Target = $typ;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl DerefMut for $newtyp {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

    };
    ($(($typ:ty, $enum_variant:path)$(,)?)*) => {
        $(impl_newtype_conv!($typ, $enum_variant);)*
    };
}

impl_newtype_conv!(
    (u8, tag::Byte),
    (i16, tag::Short),
    (i32, tag::Int),
    (i64, tag::Long),
    (f32, tag::Float),
    (f64, tag::Double),
    (String, tag::String),
    (HashMap<String, NBTTag>, tag::Compound),
    (Vec<NBTTag>, tag::List),
    (Vec<u8>, tag::ByteArray),
    (Vec<i32>, tag::IntArray),
    (Vec<i64>, tag::LongArray),
);
