//! See [NbtError].
use std::collections::LinkedList;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::string::FromUtf8Error;

/// Wrapper type for results with [NbtError].
pub type Res<T> = Result<T, NbtError>;

/// An error that can occur when reading or writing NBT from/to a buffer.
#[derive(Debug)]
pub enum NbtError {
    /// Returned when NBT could be read/written but some data is inconsistent with the NBT
    /// specification.
    ParseError(String),
    /// Returned in various circumstances when data could not be read or written.
    IoError(Box<dyn Error>),
}

impl From<FromUtf8Error> for NbtError {
    fn from(value: FromUtf8Error) -> Self {
        NbtError::IoError(Box::new(value))
    }
}

impl Display for NbtError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NbtError::ParseError(s) => {
                f.write_str(format!("could not encode/decode NBT: {}", s).as_str())
            }
            NbtError::IoError(err) => f.write_str(
                format!("encountered IO error while encoding/decoding NBT: {}", err).as_str(),
            ),
        }
    }
}

impl Error for NbtError {
    fn cause(&self) -> Option<&dyn Error> {
        if let Self::IoError(err) = &self {
            Some(err.as_ref())
        } else {
            None
        }
    }
}

/// A generic wrapper that gives a [Path] to an error type.
pub struct ErrorPath<'a, I> {
    /// The inner element that the wrapper wraps around.
    pub inner: I,
    /// The associated path. Usually, this should be the location where the error occurred.
    pub path: Path<'a>,
}

impl<'a, I> ErrorPath<'a, I> {
    /// Create a new [ErrorPath] wrapper from the inner element, using the default (empty) path.
    pub fn new(inner: I) -> Self {
        Self {
            inner,
            path: Default::default(),
        }
    }

    /// Create a new [ErrorPath] wrapper from the inner element and a path.
    pub fn new_with_path(inner: I, path: Path<'a>) -> Self {
        Self { inner, path }
    }
}

impl<'a, I: Error + 'static> Error for ErrorPath<'a, I> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.inner)
    }
}

impl<'a, I: Clone> Clone for ErrorPath<'a, I> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            path: self.path.clone(),
        }
    }
}

impl<'a, I: Default> Default for ErrorPath<'a, I> {
    fn default() -> Self {
        Self {
            inner: Default::default(),
            path: Default::default(),
        }
    }
}

impl<'a, I: Debug> Debug for ErrorPath<'a, I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ErrorPath")
            .field("inner", &self.inner)
            .field("path", &self.path)
            .finish()
    }
}

impl<'a, I: Display> Display for ErrorPath<'a, I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("`")?;
        <Path as Display>::fmt(&self.path, f)?;
        f.write_str("`: ")?;
        self.inner.fmt(f)
    }
}

impl<'a, I: PartialEq> PartialEq for ErrorPath<'a, I> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner && self.path == other.path
    }
}

impl<'a, I: Eq> Eq for ErrorPath<'a, I> {}

/// A 'path' in a rust type that indicates where an error occurred.
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Path<'a>(pub LinkedList<PathPart<'a>>);

impl<'a> Display for Path<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.0.is_empty() {
            return f.write_str("(root)");
        }

        let mut iter = self.0.iter();
        // Unwrapping will never panic here to the 'is empty' check.
        <PathPart as Display>::fmt(iter.next().unwrap(), f)?;

        for next in iter {
            if let PathPart::Element(_) = next {
                // Dont write a `.` for sequence elements.
            } else {
                f.write_str(".")?;
            }
            <PathPart as Display>::fmt(next, f)?;
        }
        Ok(())
    }
}

/// A single part of an [Path].
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PathPart<'a> {
    /// The path part is a field in a struct or a key of a map.
    Field(&'a str),
    /// THe path part is a field of a tuple.
    TupleField(usize),
    /// The path part is a sequence element.
    Element(usize),
}

impl<'a> Display for PathPart<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PathPart::Field(v) => f.write_str(*v),
            PathPart::Element(v) => {
                f.write_str("[")?;
                f.write_str(&v.to_string())?;
                f.write_str("]")
            }
            PathPart::TupleField(v) => f.write_str(&v.to_string()),
        }
    }
}
