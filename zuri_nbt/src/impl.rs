//! Implementations for type conversions from and to [NBTTag] using [From] and [TryFrom] and other
//! useful traits and methods.
use crate::decode::Reader;
use crate::encode::Writer;
use crate::err::{ErrorPath, ReadError};
use crate::{decode, encode, tag, NBTTag, NBTTagType};
use bytes::{Buf, BufMut};
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
    (Vec<u8>, tag::ByteArray),
    (Vec<i32>, tag::IntArray),
    (Vec<i64>, tag::LongArray),
);

/// Special case: converting `&str` to a [tag::String] requires a clone.
impl From<&str> for tag::String {
    fn from(value: &str) -> Self {
        tag::String(value.to_string())
    }
}

impl<T: Into<NBTTag>> From<Vec<T>> for tag::List {
    fn from(value: Vec<T>) -> Self {
        tag::List(value.into_iter().map(|v| v.into()).collect())
    }
}

impl From<tag::List> for Vec<NBTTag> {
    fn from(value: tag::List) -> Self {
        value.0
    }
}

impl Deref for tag::List {
    type Target = Vec<NBTTag>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for tag::List {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

macro_rules! impl_tagtype {
    ($typ:ty, $enum_variant:path, $enum_variant2:path) => {
        impl $typ {
            /// Returns the [NBTTagType] associated with this tag.
            #[inline]
            pub fn tag_type(&self) -> NBTTagType {
                $enum_variant
            }
        }

        impl $typ {
            /// Attempts to read the data from a buffer into an NBT value using the specified
            /// [Reader] encoding.
            ///
            /// Returns an error if the 'parent' nbt tag is not the same type as the type this
            /// method was called on.
            pub fn read(buf: &mut impl Buf, r: &mut impl Reader) -> decode::Res<Self> {
                let nbt = NBTTag::read(buf, r)?;
                let typ = nbt.tag_type().clone();
                if let $enum_variant2(t) = nbt {
                    Ok(t)
                } else {
                    let t = $enum_variant;
                    Err(ErrorPath::new(ReadError::UnexpectedTag(
                        t.to_string(),
                        typ.to_string(),
                    )))
                }
            }

            /// Attempts to write the NBT data into a buffer using the specified [Writer] encoding.
            ///
            /// Consumes the tag, as it would otherwise require a copy.
            pub fn write(self, buf: &mut impl BufMut, w: &mut impl Writer) -> encode::Res {
                let nbt: NBTTag = self.into();
                nbt.write(buf, w)
            }
        }
    };
    ($(($typ:ty, $enum_variant:path, $enum_variant2:path)$(,)?)*) => {
        $(impl_tagtype!($typ, $enum_variant, $enum_variant2);)*
    };
}

impl_tagtype!(
    (tag::Byte, NBTTagType::Byte, NBTTag::Byte),
    (tag::Short, NBTTagType::Short, NBTTag::Short),
    (tag::Int, NBTTagType::Int, NBTTag::Int),
    (tag::Long, NBTTagType::Long, NBTTag::Long),
    (tag::Float, NBTTagType::Float, NBTTag::Float),
    (tag::Double, NBTTagType::Double, NBTTag::Double),
    (tag::String, NBTTagType::String, NBTTag::String),
    (tag::Compound, NBTTagType::Compound, NBTTag::Compound),
    (tag::List, NBTTagType::List, NBTTag::List),
    (tag::ByteArray, NBTTagType::ByteArray, NBTTag::ByteArray),
    (tag::IntArray, NBTTagType::IntArray, NBTTag::IntArray),
    (tag::LongArray, NBTTagType::LongArray, NBTTag::LongArray),
);
