//! See [NbtError].
use std::error::Error;
use std::fmt::{Display, Formatter};
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
