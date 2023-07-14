#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

use std::collections::HashMap;
use std::fmt::Debug;

use bytes::{Buf, BufMut};
use strum_macros::{Display, IntoStaticStr};

use encode::Writer;

use crate::decode::Reader;
use crate::err::{ErrorPath, Path, PathPart, ReadError, WriteError};
use crate::view::View;

pub mod decode;
pub mod encode;
pub mod encoding;
pub mod err;
mod r#impl;
#[cfg(feature = "serde")]
pub mod serde;
pub mod tag;
pub mod view;

/// An enum representing all possible NBT data.
#[derive(Debug, Clone, PartialEq)]
pub enum NBTTag {
    /// An 8-bit unsigned integer.
    Byte(tag::Byte),
    /// A 16-bit signed integer.
    Short(tag::Short),
    /// A 32-bit signed integer.
    Int(tag::Int),
    /// A 64-bit signed integer.
    Long(tag::Long),
    /// A 32-bit floating point number.
    Float(tag::Float),
    /// A 64-bit floating point number.
    Double(tag::Double),
    /// A string of characters.
    ///
    /// Should never be larger than [i16::MAX].
    String(tag::String),
    /// A map containing zero or more key-value pairs.
    ///
    /// Each key maps to exactly one [NBTTag] of any type.
    Compound(tag::Compound),
    /// A variable-length list [NBTTag]s of the same type.
    ///
    /// Lists will fail to encode/decode should it contain values of which the type does not match
    /// the type of the first element in the list.
    List(tag::List),
    /// A variable-length array containing 8-bit unsigned integers.
    ByteArray(tag::ByteArray),
    /// A variable-length array containing 32-bit signed integers.
    IntArray(tag::IntArray),
    /// A variable-length array containing 64-bit signed integers.
    LongArray(tag::LongArray),
}

/// An enum representing all possible NBT tag types.
#[allow(missing_docs)]
#[derive(Copy, Clone, Debug, Display, IntoStaticStr, Eq, PartialEq)]
pub enum NBTTagType {
    Byte,
    Short,
    Int,
    Long,
    Float,
    Double,
    String,
    Compound,
    List,
    ByteArray,
    IntArray,
    LongArray,
}

impl NBTTag {
    /// Returns the [NBTTagType] associated with the tag variant contained in the enum.
    pub fn tag_type(&self) -> NBTTagType {
        match self {
            NBTTag::Byte(v) => v.tag_type(),
            NBTTag::Short(v) => v.tag_type(),
            NBTTag::Int(v) => v.tag_type(),
            NBTTag::Long(v) => v.tag_type(),
            NBTTag::Float(v) => v.tag_type(),
            NBTTag::Double(v) => v.tag_type(),
            NBTTag::String(v) => v.tag_type(),
            NBTTag::Compound(v) => v.tag_type(),
            NBTTag::List(v) => v.tag_type(),
            NBTTag::ByteArray(v) => v.tag_type(),
            NBTTag::IntArray(v) => v.tag_type(),
            NBTTag::LongArray(v) => v.tag_type(),
        }
    }

    /// Creates a [View] for the NBT tag for easy reading.
    pub fn view(&self) -> View {
        View::new(self)
    }

    /// Attempts to read the data from a buffer into an NBT value using the specified [Reader]
    /// encoding.
    pub fn read(buf: &mut impl Buf, r: &mut impl Reader) -> decode::Res<Self> {
        let tag_id = r.u8(buf)?;
        r.string(buf)?;
        Self::read_inner(buf, tag_id, r)
    }

    /// Attempts to write the NBT data into a buffer using the specified [Writer] encoding.
    pub fn write(&self, buf: &mut impl BufMut, w: &mut impl Writer) -> encode::Res {
        w.write_u8(buf, self.tag_id())?;
        w.write_string(buf, "")?;
        self.write_inner(buf, w)
    }

    /// Internal function used to read NBT data. Slightly differs from [Self::read].
    fn read_inner(buf: &mut impl Buf, tag_id: u8, r: &mut impl Reader) -> decode::Res<Self> {
        Ok(match tag_id {
            1 => NBTTag::Byte(r.u8(buf)?.into()),
            2 => NBTTag::Short(r.i16(buf)?.into()),
            3 => NBTTag::Int(r.i32(buf)?.into()),
            4 => NBTTag::Long(r.i64(buf)?.into()),
            5 => NBTTag::Float(r.f32(buf)?.into()),
            6 => NBTTag::Double(r.f64(buf)?.into()),
            8 => NBTTag::String(r.string(buf)?.into()),
            10 => {
                let mut map = HashMap::new();
                loop {
                    let content_type = r.u8(buf)?;
                    if content_type == 0 {
                        break;
                    }
                    let name = r.string(buf)?;
                    let value = Self::read_inner(buf, content_type, r)
                        .map_err(|err| err.prepend(PathPart::MapKey(name.clone())))?;
                    map.insert(name, value);
                }
                NBTTag::Compound(map.into())
            }
            9 => {
                let content_type = r.u8(buf)?;
                let len = r.i32(buf)?;
                if len < 0 {
                    return Err(ErrorPath::new(ReadError::SeqLengthViolation(
                        i32::MAX as usize,
                        len as usize,
                    )));
                }
                let mut vec = Vec::with_capacity(len as usize);
                for i in 0..len {
                    vec.push(
                        Self::read_inner(buf, content_type, r)
                            .map_err(|err| err.prepend(PathPart::Element(i as usize)))?,
                    );
                }
                NBTTag::List(vec.into())
            }
            7 => NBTTag::ByteArray(r.u8_vec(buf)?.into()),
            11 => NBTTag::IntArray(r.i32_vec(buf)?.into()),
            12 => NBTTag::LongArray(r.i64_vec(buf)?.into()),
            _ => panic!("Unknown tag type {}", tag_id),
        })
    }

    /// Internal function used to write NBT data. Slightly differs from [Self::write].
    fn write_inner(&self, buf: &mut impl BufMut, w: &mut impl Writer) -> encode::Res {
        match self {
            Self::Byte(x) => w.write_u8(buf, x.0)?,
            Self::Short(x) => w.write_i16(buf, x.0)?,
            Self::Int(x) => w.write_i32(buf, x.0)?,
            Self::Long(x) => w.write_i64(buf, x.0)?,
            Self::Float(x) => w.write_f32(buf, x.0)?,
            Self::Double(x) => w.write_f64(buf, x.0)?,
            Self::String(x) => w.write_string(buf, x.0.as_str())?,
            Self::Compound(x) => {
                for (name, val) in &x.0 {
                    w.write_u8(buf, val.tag_id())?;
                    w.write_string(buf, name)?;
                    val.write_inner(buf, w)?;
                }
                w.write_end(buf)?;
            }
            Self::List(x) => {
                let first_id = if x.0.is_empty() {
                    NBTTag::Byte(0.into()).tag_id()
                } else {
                    x.0[0].tag_id()
                };

                w.write_u8(buf, first_id)?;
                w.write_i32(buf, x.len() as i32)?;
                for (i, v) in x.0.iter().enumerate() {
                    if v.tag_id() != first_id {
                        return Err(ErrorPath::new_with_path(
                            WriteError::UnexpectedTag(
                                x[0].tag_type().to_string(),
                                v.tag_type().to_string(),
                            ),
                            Path::from_single(PathPart::Element(i)),
                        ));
                    }
                    v.write_inner(buf, w)?;
                }
            }
            Self::ByteArray(x) => w.write_u8_vec(buf, &x.0)?,
            Self::IntArray(x) => w.write_i32_vec(buf, &x.0)?,
            Self::LongArray(x) => w.write_i64_vec(buf, &x.0)?,
        };
        Ok(())
    }

    /// Gets the discriminator of a [NBTTag]'s type used for encoding and decoding.
    pub(crate) fn tag_id(&self) -> u8 {
        match self {
            NBTTag::Byte(_) => 1,
            NBTTag::Short(_) => 2,
            NBTTag::Int(_) => 3,
            NBTTag::Long(_) => 4,
            NBTTag::Float(_) => 5,
            NBTTag::Double(_) => 6,
            NBTTag::String(_) => 8,
            NBTTag::Compound(_) => 10,
            NBTTag::List(_) => 9,
            NBTTag::ByteArray(_) => 7,
            NBTTag::IntArray(_) => 11,
            NBTTag::LongArray(_) => 12,
        }
    }
}

impl Default for NBTTag {
    fn default() -> Self {
        Self::Compound(HashMap::new().into())
    }
}
