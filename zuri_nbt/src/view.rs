//! The [View] struct is a utility for easily traversing and reading NBT data in an efficient manner
//! and without too much boilerplate.
//!
//! # Usage
//! ```
//! # use zuri_nbt::{NBTTag, NBTTagType};
//! # use zuri_nbt::view::ViewError;
//! let nbt: NBTTag;
//! # nbt = NBTTag::Compound(Default::default());
//! // Read a string tag at `nbt.foo[3].bar`, if all these tags exist.
//! let result = nbt.view().at("foo").at(3).at("bar").string();
//! # assert_eq!(result, Err(ViewError::MissingTag(3)));
//! // Check if the nbt tag is any flaot tag (and return the value).
//! let result = nbt.view().any_float();
//! # assert_eq!(result, Err(ViewError::MismatchedType {
//! #    expected: NBTTagType::Double,
//! #    found: NBTTagType::Compound,
//! # }));
//! ```

use std::borrow::Cow;
use std::collections::hash_map;
use std::slice;

use thiserror::Error;

use crate::{tag, NBTTag, NBTTagType};

/// A reference to NBT data allowing for easy reading of said data.
///
/// This type is always cheap to clone. It will never perform any heap allocations.
#[derive(Debug, Clone)]
pub struct View<'a> {
    tag: InnerView<'a>,
}

/// An iterator over an [NBTTag] in a view.
///
/// Iterates over all values in the underlying container NBT tag. If the underlying NBT tag is not a
/// container then, by default, it will include that NBT tag once in the returned iterator.
#[derive(Debug, Clone)]
pub struct ViewIterator<'a>(InnerViewIterator<'a>);

/// An error returned when trying to turn a [View] into a concrete value. Displays what went wrong
/// during reading.
#[derive(Debug, Error, Clone, Eq, PartialEq)]
pub enum ViewError {
    /// The target tag and possibly one or more parent tags could not be found.
    ///
    /// Contains an integer denoting how many index's ago the last valid tag was. This starts out at
    /// 1, and increases by 1 every time a [View::at], [View::at_index] or [View::at_key] call is
    /// performed on an invalid view.
    #[error("missing tag")]
    MissingTag(usize),
    /// The tag was found, but is a different type than expected.
    #[error("expected tag of type `{expected}`, found type `{found}`")]
    MismatchedType {
        /// The tag type that the read tag was expected to be by the caller.
        expected: NBTTagType,
        /// The actual tag type found in the NBT data.
        found: NBTTagType,
    },
}

/// Helper type to make the [View::at] method possible.
#[derive(Debug, Clone)]
pub enum ViewIndex<'a> {
    /// An index for a value in a compound tag.
    CompoundIndex(Cow<'a, str>),
    /// An index for a value in a list tag or any array tag.
    ListIndex(usize),
}

impl<'a> From<&'a NBTTag> for View<'a> {
    fn from(value: &'a NBTTag) -> Self {
        Self::new(value)
    }
}

impl<'a> View<'a> {
    /// Creates a new view from an NBT tag.
    pub fn new(tag: &'a NBTTag) -> Self {
        Self {
            tag: InnerView::Ok(Cow::Borrowed(tag)),
        }
    }

    /// Gets the underlying [NBTTag] that the view points to, if present.
    pub fn get(&self) -> Option<&NBTTag> {
        match &self.tag {
            InnerView::Ok(v) => Some(v.as_ref()),
            InnerView::NotFound(_) => None,
        }
    }

    /// Returns true if the view points to a valid NBT tag.
    pub fn valid(&self) -> bool {
        match self.tag {
            InnerView::Ok(_) => true,
            InnerView::NotFound(_) => false,
        }
    }

    /// Returns true if the underlying NBT tag is empty.
    ///
    /// Since a view can point to a non-container tag, it is important to note that this will
    /// consider non-container tags (int, string, ...) as being non-empty.
    ///
    /// Returns true if the view does not point to any NBT tag.
    pub fn is_empty(&self) -> bool {
        match self.get() {
            None => true,
            Some(NBTTag::Compound(v)) => v.is_empty(),
            Some(NBTTag::List(v)) => v.is_empty(),
            Some(NBTTag::ByteArray(v)) => v.is_empty(),
            Some(NBTTag::IntArray(v)) => v.is_empty(),
            Some(NBTTag::LongArray(v)) => v.is_empty(),
            Some(_) => false,
        }
    }

    /// Returns true if the underlying NBT tag is a container tag (a tag that may contain other
    /// tags).
    ///
    /// Examples of such tags are [tag::Compound], [tag::List] and [tag::ByteArray]. Note that
    /// [tag::String] is not considered a container tag!
    ///
    /// Returns false if the view does not point to any NBT tag.
    pub fn is_container(&self) -> bool {
        match self.get() {
            Some(
                NBTTag::Compound(_)
                | NBTTag::List(_)
                | NBTTag::ByteArray(_)
                | NBTTag::IntArray(_)
                | NBTTag::LongArray(_),
            ) => true,
            _ => false,
        }
    }

    /// Get an entry from the underlying tag by its index.
    ///
    /// Runs either [Self::at_key] if the index is a [ViewIndex::CompoundIndex] or [Self::at_index]
    /// if the index is a [ViewIndex::ListIndex].
    pub fn at<'b>(&self, index: impl Into<ViewIndex<'b>>) -> Self {
        match index.into() {
            ViewIndex::CompoundIndex(v) => self.at_key(v.as_ref()),
            ViewIndex::ListIndex(v) => self.at_index(v),
        }
    }

    /// Get an entry in the underlying [tag::Compound] tag by key.
    ///
    /// If the key was not found, or the underlying NBT tag is not a compound tag, the returned view
    /// will simply point to nothing.
    pub fn at_key(&self, key: impl AsRef<str>) -> Self {
        match self.tag {
            InnerView::Ok(Cow::Borrowed(NBTTag::Compound(v))) => Self {
                tag: InnerView::from_opt(v.get(key.as_ref()).map(|v| Cow::Borrowed(v))),
            },
            InnerView::Ok(Cow::Owned(NBTTag::Compound(_))) => unreachable!(),
            InnerView::Ok(_) => Self {
                tag: InnerView::NotFound(1),
            },
            InnerView::NotFound(v) => Self {
                tag: InnerView::NotFound(v + 1),
            },
        }
    }

    /// Get an entry in the underlying [tag::List], [tag::ByteArray], [tag::IntArray],
    /// [tag::LongArray] tag by index.
    ///
    /// If the key was not found, or the underlying NBT tag is not a list tag or array tag, the
    /// returned view will simply point to nothing.
    pub fn at_index(&self, index: usize) -> Self {
        Self {
            tag: match &self.tag {
                InnerView::Ok(Cow::Borrowed(NBTTag::List(v))) => {
                    InnerView::from_opt(v.get(index).map(|v| Cow::Borrowed(v)))
                }
                InnerView::Ok(Cow::Borrowed(NBTTag::ByteArray(v))) => InnerView::from_opt(
                    v.get(index)
                        .map(|v| NBTTag::Byte(tag::Byte(*v)))
                        .map(|v| Cow::Owned(v)),
                ),
                InnerView::Ok(Cow::Borrowed(NBTTag::IntArray(v))) => InnerView::from_opt(
                    v.get(index)
                        .map(|v| NBTTag::Int(tag::Int(*v)))
                        .map(|v| Cow::Owned(v)),
                ),
                InnerView::Ok(Cow::Borrowed(NBTTag::LongArray(v))) => InnerView::from_opt(
                    v.get(index)
                        .map(|v| NBTTag::Long(tag::Long(*v)))
                        .map(|v| Cow::Owned(v)),
                ),
                InnerView::Ok(Cow::Owned(NBTTag::List(_))) => unreachable!(),
                InnerView::Ok(Cow::Owned(NBTTag::ByteArray(_))) => unreachable!(),
                InnerView::Ok(Cow::Owned(NBTTag::IntArray(_))) => unreachable!(),
                InnerView::Ok(Cow::Owned(NBTTag::LongArray(_))) => unreachable!(),
                InnerView::Ok(_) => InnerView::NotFound(1),
                InnerView::NotFound(v) => InnerView::NotFound(v + 1),
            },
        }
    }

    /// Reads the current tag pointed to. If this tag is any integer tag, it is converted to an i64
    /// and returned.
    pub fn any_int(&self) -> Result<i64, ViewError> {
        match &self.tag {
            InnerView::Ok(Cow::Borrowed(NBTTag::Long(s))) => Ok(s.0.clone()),
            InnerView::Ok(Cow::Borrowed(NBTTag::Int(s))) => Ok(s.0.clone() as i64),
            InnerView::Ok(Cow::Borrowed(NBTTag::Short(s))) => Ok(s.0.clone() as i64),
            InnerView::Ok(Cow::Borrowed(NBTTag::Byte(s))) => Ok(s.0.clone() as i64),
            InnerView::Ok(Cow::Owned(NBTTag::Long(s))) => Ok(s.0.clone()),
            InnerView::Ok(Cow::Owned(NBTTag::Int(s))) => Ok(s.0.clone() as i64),
            InnerView::Ok(Cow::Owned(NBTTag::Short(_))) => unreachable!(),
            InnerView::Ok(Cow::Owned(NBTTag::Byte(s))) => Ok(s.0.clone() as i64),
            InnerView::Ok(t) => Err(ViewError::MismatchedType {
                expected: NBTTagType::Long,
                found: t.tag_type(),
            }),
            InnerView::NotFound(v) => Err(ViewError::MissingTag(*v)),
        }
    }

    /// Reads the current tag pointed to. If this tag is any floating point tag, it is converted to
    /// an f64 and returned.
    pub fn any_float(&self) -> Result<f64, ViewError> {
        match &self.tag {
            InnerView::Ok(Cow::Borrowed(NBTTag::Double(s))) => Ok(s.0.clone()),
            InnerView::Ok(Cow::Borrowed(NBTTag::Float(s))) => Ok(s.0.clone() as f64),
            InnerView::Ok(Cow::Owned(NBTTag::Double(_))) => unreachable!(),
            InnerView::Ok(Cow::Owned(NBTTag::Float(_))) => unreachable!(),
            InnerView::Ok(t) => Err(ViewError::MismatchedType {
                expected: NBTTagType::Double,
                found: t.tag_type(),
            }),
            InnerView::NotFound(v) => Err(ViewError::MissingTag(*v)),
        }
    }

    /// Returns a reference to a [tag::Compound] if the underlying tag's type matches this.
    pub fn compound(&self) -> Result<&'a tag::Compound, ViewError> {
        match &self.tag {
            InnerView::Ok(Cow::Borrowed(NBTTag::Compound(s))) => Ok(&s),
            InnerView::Ok(Cow::Owned(NBTTag::Compound(_))) => unreachable!(),
            InnerView::Ok(t) => Err(ViewError::MismatchedType {
                expected: NBTTagType::Compound,
                found: t.tag_type(),
            }),
            InnerView::NotFound(v) => Err(ViewError::MissingTag(*v)),
        }
    }

    /// Returns the string value of the tag if the underlying tag is a [tag::String].
    pub fn string(&self) -> Result<&'a str, ViewError> {
        match &self.tag {
            InnerView::Ok(Cow::Borrowed(NBTTag::String(s))) => Ok(s.as_str()),
            InnerView::Ok(Cow::Owned(NBTTag::String(_))) => unreachable!(),
            InnerView::Ok(t) => Err(ViewError::MismatchedType {
                expected: NBTTagType::String,
                found: t.tag_type(),
            }),
            InnerView::NotFound(v) => Err(ViewError::MissingTag(*v)),
        }
    }

    /// Returns the byte value of the tag if the underlying tag is a [tag::Byte].
    pub fn byte(&self) -> Result<u8, ViewError> {
        match &self.tag {
            InnerView::Ok(Cow::Borrowed(NBTTag::Byte(s))) => Ok(s.0.clone()),
            InnerView::Ok(Cow::Owned(NBTTag::Byte(s))) => Ok(s.0.clone()),
            InnerView::Ok(t) => Err(ViewError::MismatchedType {
                expected: NBTTagType::Byte,
                found: t.tag_type(),
            }),
            InnerView::NotFound(v) => Err(ViewError::MissingTag(*v)),
        }
    }

    /// Returns the short value of the tag if the underlying tag is a [tag::Short].
    pub fn short(&self) -> Result<i16, ViewError> {
        match &self.tag {
            InnerView::Ok(Cow::Borrowed(NBTTag::Short(s))) => Ok(s.0.clone()),
            InnerView::Ok(Cow::Owned(NBTTag::Short(_))) => unreachable!(),
            InnerView::Ok(t) => Err(ViewError::MismatchedType {
                expected: NBTTagType::Short,
                found: t.tag_type(),
            }),
            InnerView::NotFound(v) => Err(ViewError::MissingTag(*v)),
        }
    }

    /// Returns the int value of the tag if the underlying tag is a [tag::Int].
    pub fn int(&self) -> Result<i32, ViewError> {
        match &self.tag {
            InnerView::Ok(Cow::Borrowed(NBTTag::Int(s))) => Ok(s.0.clone()),
            InnerView::Ok(Cow::Owned(NBTTag::Int(s))) => Ok(s.0.clone()),
            InnerView::Ok(t) => Err(ViewError::MismatchedType {
                expected: NBTTagType::Int,
                found: t.tag_type(),
            }),
            InnerView::NotFound(v) => Err(ViewError::MissingTag(*v)),
        }
    }

    /// Returns the long value of the tag if the underlying tag is a [tag::Long].
    pub fn long(&self) -> Result<i64, ViewError> {
        match &self.tag {
            InnerView::Ok(Cow::Borrowed(NBTTag::Long(s))) => Ok(s.0.clone()),
            InnerView::Ok(Cow::Owned(NBTTag::Long(s))) => Ok(s.0.clone()),
            InnerView::Ok(t) => Err(ViewError::MismatchedType {
                expected: NBTTagType::Long,
                found: t.tag_type(),
            }),
            InnerView::NotFound(v) => Err(ViewError::MissingTag(*v)),
        }
    }

    /// Returns the float value of the tag if the underlying tag is a [tag::Float].
    pub fn float(&self) -> Result<f32, ViewError> {
        match &self.tag {
            InnerView::Ok(Cow::Borrowed(NBTTag::Float(s))) => Ok(s.0.clone()),
            InnerView::Ok(Cow::Owned(NBTTag::Float(_))) => unreachable!(),
            InnerView::Ok(t) => Err(ViewError::MismatchedType {
                expected: NBTTagType::Float,
                found: t.tag_type(),
            }),
            InnerView::NotFound(v) => Err(ViewError::MissingTag(*v)),
        }
    }

    /// Returns the double value of the tag if the underlying tag is a [tag::Double].
    pub fn double(&self) -> Result<f64, ViewError> {
        match &self.tag {
            InnerView::Ok(Cow::Borrowed(NBTTag::Double(s))) => Ok(s.0.clone()),
            InnerView::Ok(Cow::Owned(NBTTag::Double(_))) => unreachable!(),
            InnerView::Ok(t) => Err(ViewError::MismatchedType {
                expected: NBTTagType::Double,
                found: t.tag_type(),
            }),
            InnerView::NotFound(v) => Err(ViewError::MissingTag(*v)),
        }
    }

    /// Iterates over all entries in the view's container, or the underlying tag itself if it is not
    /// a container.
    ///
    /// See [ViewIterator] for more info.
    ///
    /// If the view is not valid, an empty iterator is returned.
    pub fn iter(&'a self) -> ViewIterator<'a> {
        match &self.tag {
            InnerView::Ok(Cow::Borrowed(NBTTag::Compound(v))) => {
                ViewIterator(InnerViewIterator::Compound(v.iter()))
            }
            InnerView::Ok(Cow::Borrowed(NBTTag::List(v))) => {
                ViewIterator(InnerViewIterator::List(v.iter()))
            }
            InnerView::Ok(Cow::Borrowed(NBTTag::ByteArray(v))) => {
                ViewIterator(InnerViewIterator::ByteArray(v.iter()))
            }
            InnerView::Ok(Cow::Borrowed(NBTTag::IntArray(v))) => {
                ViewIterator(InnerViewIterator::IntArray(v.iter()))
            }
            InnerView::Ok(Cow::Borrowed(NBTTag::LongArray(v))) => {
                ViewIterator(InnerViewIterator::LongArray(v.iter()))
            }
            InnerView::Ok(Cow::Owned(NBTTag::Compound(_))) => unreachable!(),
            InnerView::Ok(Cow::Owned(NBTTag::List(_))) => unreachable!(),
            InnerView::Ok(Cow::Owned(NBTTag::ByteArray(_))) => unreachable!(),
            InnerView::Ok(Cow::Owned(NBTTag::IntArray(_))) => unreachable!(),
            InnerView::Ok(Cow::Owned(NBTTag::LongArray(_))) => unreachable!(),
            InnerView::Ok(v) => ViewIterator(InnerViewIterator::Single(v.as_ref())),
            InnerView::NotFound(_) => ViewIterator(InnerViewIterator::None),
        }
    }

    /// Iterates over all entries in the view's underlying container tag, returning an empty
    /// iterator if the underlying tag is not a container.
    ///
    /// See [Self::iter] for more info.
    pub fn iter_container(&'a self) -> ViewIterator<'a> {
        let mut iter = self.iter();
        if let InnerViewIterator::Single(_) = &iter.0 {
            iter.0 = InnerViewIterator::None;
        }
        iter
    }

    /// Iterate over entries in the underlying list tag, if the view points to one.
    pub fn iter_list(&self) -> ViewIterator<'a> {
        match &self.tag {
            InnerView::Ok(Cow::Borrowed(NBTTag::List(v))) => {
                ViewIterator(InnerViewIterator::List(v.iter()))
            }
            InnerView::Ok(Cow::Owned(NBTTag::List(_))) => unreachable!(),
            _ => ViewIterator(InnerViewIterator::None),
        }
    }

    /// Iterate over entries in the underlying compound tag, if the view points to one.
    pub fn iter_compound(&self) -> impl Iterator<Item = (&'a str, &'a NBTTag)> {
        self.compound()
            .map(|v| v.iter().map(|(k, v)| (k.as_str(), v)))
            .into_iter()
            .flatten()
    }

    /// Maps the underlying NBT tag to a value of a different type if the view points to a valid NBT
    /// tag.
    pub fn map<F, T>(&'a self, f: F) -> Option<T>
    where
        F: FnOnce(&'a NBTTag) -> T,
    {
        self.get().map(f)
    }
}

impl<'a> IntoIterator for &'a View<'a> {
    type Item = View<'a>;
    type IntoIter = ViewIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> Iterator for ViewIterator<'a> {
    type Item = View<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            InnerViewIterator::Compound(v) => v.next().map(|(_, v)| View::new(v)),
            InnerViewIterator::List(v) => v.next().map(|v| View::new(v)),
            InnerViewIterator::ByteArray(v) => v.next().map(|v| View {
                tag: InnerView::Ok(Cow::Owned(NBTTag::Byte(tag::Byte(*v)))),
            }),
            InnerViewIterator::IntArray(v) => v.next().map(|v| View {
                tag: InnerView::Ok(Cow::Owned(NBTTag::Int(tag::Int(*v)))),
            }),
            InnerViewIterator::LongArray(v) => v.next().map(|v| View {
                tag: InnerView::Ok(Cow::Owned(NBTTag::Long(tag::Long(*v)))),
            }),
            InnerViewIterator::Single(v) => {
                let r = Some(View {
                    tag: InnerView::Ok(Cow::Borrowed(*v)),
                });
                self.0 = InnerViewIterator::None;
                r
            }
            InnerViewIterator::None => None,
        }
    }
}

#[derive(Debug, Clone)]
enum InnerViewIterator<'a> {
    Compound(hash_map::Iter<'a, String, NBTTag>),
    List(slice::Iter<'a, NBTTag>),
    ByteArray(slice::Iter<'a, u8>),
    IntArray(slice::Iter<'a, i32>),
    LongArray(slice::Iter<'a, i64>),
    Single(&'a NBTTag),
    None,
}

impl<'a> From<&'a str> for ViewIndex<'a> {
    fn from(value: &'a str) -> Self {
        Self::CompoundIndex(Cow::Borrowed(value))
    }
}

impl<'a> From<String> for ViewIndex<'a> {
    fn from(value: String) -> Self {
        Self::CompoundIndex(Cow::Owned(value))
    }
}

impl<'a> From<usize> for ViewIndex<'a> {
    fn from(value: usize) -> Self {
        Self::ListIndex(value)
    }
}

#[derive(Debug, Clone)]
enum InnerView<'a> {
    Ok(Cow<'a, NBTTag>),
    NotFound(usize),
}

impl<'a> InnerView<'a> {
    fn from_opt(o: Option<Cow<'a, NBTTag>>) -> Self {
        match o {
            Some(v) => InnerView::Ok(v),
            None => InnerView::NotFound(1),
        }
    }
}
