//! The [View] struct is a utility for easily traversing and reading NBT data in an efficient manner
//! and without too much boilerplate.

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
    pub(super) tag: Option<Cow<'a, NBTTag>>,
}

/// An iterator over an [NBTTag] in a view.
///
/// Iterates over all values in the underlying container NBT tag. If the underlying NBT tag is not a
/// container then, by default, it will include that NBT tag once in the returned iterator.
#[derive(Debug, Clone)]
pub struct ViewIterator<'a>(InnerViewIterator<'a>);

/// An error returned when trying to turn a [View] into a concrete value. Displays what went wrong
/// during reading.
#[derive(Debug, Error, Clone)]
pub enum ViewError {
    /// The target tag and possibly one or more parent tags could not be found.
    #[error("missing tag`")]
    MissingTag,
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
        Self {
            tag: Some(Cow::Borrowed(value)),
        }
    }
}

impl<'a> View<'a> {
    /// Gets the underlying [NBTTag] that the view points to, if present.
    pub fn get(&'a self) -> Option<&'a NBTTag> {
        self.tag.as_ref().map(|v| v.as_ref())
    }

    /// Returns true if the view points to a valid NBT tag.
    pub fn valid(&self) -> bool {
        self.tag.is_some()
    }

    /// Returns true if the underlying NBT tag is empty.
    ///
    /// Since a view can point to a non-container tag, it is important to note that this will
    /// consider non-container tags (int, string, ...) as being non-empty.
    ///
    /// Returns true if the view does not point to any NBT tag.
    pub fn is_empty(&'a self) -> bool {
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
    pub fn is_container(&'a self) -> bool {
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
    pub fn at<'b>(&'a self, index: impl Into<ViewIndex<'b>>) -> Self {
        match index.into() {
            ViewIndex::CompoundIndex(v) => self.at_key(v.as_ref()),
            ViewIndex::ListIndex(v) => self.at_index(v),
        }
    }

    /// Get an entry in the underlying [tag::Compound] tag by key.
    ///
    /// If the key was not found, or the underlying NBT tag is not a compound tag, the returned view
    /// will simply point to nothing.
    pub fn at_key(&'a self, key: impl AsRef<str>) -> Self {
        match self.get() {
            Some(NBTTag::Compound(v)) => Self {
                tag: v.get(key.as_ref()).map(|v| Cow::Borrowed(v)),
            },
            _ => Self { tag: None },
        }
    }

    /// Get an entry in the underlying [tag::List], [tag::ByteArray], [tag::IntArray],
    /// [tag::LongArray] tag by index.
    ///
    /// If the key was not found, or the underlying NBT tag is not a list tag or array tag, the
    /// returned view will simply point to nothing.
    pub fn at_index(&'a self, index: usize) -> Self {
        Self {
            tag: match self.get() {
                Some(NBTTag::List(v)) => v.get(index).map(|v| Cow::Borrowed(v)),
                Some(NBTTag::ByteArray(v)) => v
                    .get(index)
                    .map(|v| NBTTag::Byte(tag::Byte(*v)))
                    .map(|v| Cow::Owned(v)),
                Some(NBTTag::IntArray(v)) => v
                    .get(index)
                    .map(|v| NBTTag::Int(tag::Int(*v)))
                    .map(|v| Cow::Owned(v)),
                Some(NBTTag::LongArray(v)) => v
                    .get(index)
                    .map(|v| NBTTag::Long(tag::Long(*v)))
                    .map(|v| Cow::Owned(v)),
                _ => None,
            },
        }
    }

    /// Reads the current tag pointed to. If this tag is any integer tag, it is converted to an i64
    /// and returned.
    pub fn any_int(&'a self) -> Result<i64, ViewError> {
        match self.get() {
            Some(NBTTag::Long(s)) => Ok(s.0.clone()),
            Some(NBTTag::Int(s)) => Ok(s.0.clone() as i64),
            Some(NBTTag::Short(s)) => Ok(s.0.clone() as i64),
            Some(NBTTag::Byte(s)) => Ok(s.0.clone() as i64),
            Some(t) => Err(ViewError::MismatchedType {
                expected: NBTTagType::Long,
                found: t.tag_type(),
            }),
            None => Err(ViewError::MissingTag),
        }
    }

    /// Reads the current tag pointed to. If this tag is any floating point tag, it is converted to
    /// an f64 and returned.
    pub fn any_float(&'a self) -> Result<f64, ViewError> {
        match self.get() {
            Some(NBTTag::Double(s)) => Ok(s.0.clone()),
            Some(NBTTag::Float(s)) => Ok(s.0.clone() as f64),
            Some(t) => Err(ViewError::MismatchedType {
                expected: NBTTagType::Double,
                found: t.tag_type(),
            }),
            None => Err(ViewError::MissingTag),
        }
    }

    /// Returns a reference to a [tag::Compound] if the underlying tag's type matches this.
    pub fn compound(&self) -> Result<&'a tag::Compound, ViewError> {
        match &self.tag {
            Some(Cow::Borrowed(NBTTag::Compound(s))) => Ok(&s),
            Some(Cow::Owned(NBTTag::Compound(_))) => unreachable!(),
            Some(t) => Err(ViewError::MismatchedType {
                expected: NBTTagType::Compound,
                found: t.tag_type(),
            }),
            None => Err(ViewError::MissingTag),
        }
    }

    /// Returns the string value of the tag if the underlying tag is a [tag::String].
    pub fn string(&'a self) -> Result<&'a str, ViewError> {
        match self.get() {
            Some(NBTTag::String(s)) => Ok(s.as_str()),
            Some(t) => Err(ViewError::MismatchedType {
                expected: NBTTagType::String,
                found: t.tag_type(),
            }),
            None => Err(ViewError::MissingTag),
        }
    }

    /// Returns the byte value of the tag if the underlying tag is a [tag::Byte].
    pub fn byte(&self) -> Result<u8, ViewError> {
        match self.get() {
            Some(NBTTag::Byte(s)) => Ok(s.0.clone()),
            Some(t) => Err(ViewError::MismatchedType {
                expected: NBTTagType::Byte,
                found: t.tag_type(),
            }),
            None => Err(ViewError::MissingTag),
        }
    }

    /// Returns the short value of the tag if the underlying tag is a [tag::Short].
    pub fn short(&self) -> Result<i16, ViewError> {
        match self.get() {
            Some(NBTTag::Short(s)) => Ok(s.0.clone()),
            Some(t) => Err(ViewError::MismatchedType {
                expected: NBTTagType::Short,
                found: t.tag_type(),
            }),
            None => Err(ViewError::MissingTag),
        }
    }

    /// Returns the int value of the tag if the underlying tag is a [tag::Int].
    pub fn int(&self) -> Result<i32, ViewError> {
        match self.get() {
            Some(NBTTag::Int(s)) => Ok(s.0.clone()),
            Some(t) => Err(ViewError::MismatchedType {
                expected: NBTTagType::Int,
                found: t.tag_type(),
            }),
            None => Err(ViewError::MissingTag),
        }
    }

    /// Returns the long value of the tag if the underlying tag is a [tag::Long].
    pub fn long(&self) -> Result<i64, ViewError> {
        match self.get() {
            Some(NBTTag::Long(s)) => Ok(s.0.clone()),
            Some(t) => Err(ViewError::MismatchedType {
                expected: NBTTagType::Long,
                found: t.tag_type(),
            }),
            None => Err(ViewError::MissingTag),
        }
    }

    /// Returns the float value of the tag if the underlying tag is a [tag::Float].
    pub fn float(&self) -> Result<f32, ViewError> {
        match self.get() {
            Some(NBTTag::Float(s)) => Ok(s.0.clone()),
            Some(t) => Err(ViewError::MismatchedType {
                expected: NBTTagType::Float,
                found: t.tag_type(),
            }),
            None => Err(ViewError::MissingTag),
        }
    }

    /// Returns the double value of the tag if the underlying tag is a [tag::Double].
    pub fn double(&self) -> Result<f64, ViewError> {
        match self.get() {
            Some(NBTTag::Double(s)) => Ok(s.0.clone()),
            Some(t) => Err(ViewError::MismatchedType {
                expected: NBTTagType::Double,
                found: t.tag_type(),
            }),
            None => Err(ViewError::MissingTag),
        }
    }

    /// Iterates over all entries in the view's container, or the underlying tag itself if it is not
    /// a container.
    ///
    /// See [ViewIterator] for more info.
    ///
    /// If the view is not valid, an empty iterator is returned.
    pub fn iter(&'a self) -> ViewIterator<'a> {
        match self.get() {
            Some(NBTTag::Compound(v)) => ViewIterator(InnerViewIterator::Compound(v.iter())),
            Some(NBTTag::List(v)) => ViewIterator(InnerViewIterator::List(v.iter())),
            Some(NBTTag::ByteArray(v)) => ViewIterator(InnerViewIterator::ByteArray(v.iter())),
            Some(NBTTag::IntArray(v)) => ViewIterator(InnerViewIterator::IntArray(v.iter())),
            Some(NBTTag::LongArray(v)) => ViewIterator(InnerViewIterator::LongArray(v.iter())),
            Some(v) => ViewIterator(InnerViewIterator::Single(v)),
            _ => ViewIterator(InnerViewIterator::None),
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
        self.tag.as_ref().map(|v| v.as_ref()).map(f)
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
            InnerViewIterator::Compound(v) => v.next().map(|(_, v)| View {
                tag: Some(Cow::Borrowed(v)),
            }),
            InnerViewIterator::List(v) => v.next().map(|v| View {
                tag: Some(Cow::Borrowed(v)),
            }),
            InnerViewIterator::ByteArray(v) => v.next().map(|v| View {
                tag: Some(Cow::Owned(NBTTag::Byte(tag::Byte(*v)))),
            }),
            InnerViewIterator::IntArray(v) => v.next().map(|v| View {
                tag: Some(Cow::Owned(NBTTag::Int(tag::Int(*v)))),
            }),
            InnerViewIterator::LongArray(v) => v.next().map(|v| View {
                tag: Some(Cow::Owned(NBTTag::Long(tag::Long(*v)))),
            }),
            InnerViewIterator::Single(v) => {
                let r = Some(View {
                    tag: Some(Cow::Borrowed(*v)),
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
