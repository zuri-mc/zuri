//! See [Reader].
use crate::err::{ErrorPath, PathPart, ReadError};
use bytes::Buf;
use std::mem;

/// A short notation for the result type used in the [Reader].
pub type Res<T> = Result<T, ErrorPath<ReadError>>;

/// A trait that can be implemented to alter how basic NBT types are read.
///
/// All the implemented methods must not panic.
pub trait Reader {
    /// Reads an 8-bit unsigned integer.
    fn u8(&mut self, buf: &mut impl Buf) -> Res<u8> {
        if buf.remaining() < mem::size_of::<u8>() {
            return Err(ErrorPath::new(ReadError::UnexpectedEOF));
        }
        Ok(buf.get_u8())
    }
    /// Reads a 16-bit signed integer.
    fn i16(&mut self, buf: &mut impl Buf) -> Res<i16>;
    /// Reads a 32-bit signed integer.
    fn i32(&mut self, buf: &mut impl Buf) -> Res<i32>;
    /// Reads a 64-bit signed integer.
    fn i64(&mut self, buf: &mut impl Buf) -> Res<i64>;
    /// Reads a 32-bit floating point number.
    fn f32(&mut self, buf: &mut impl Buf) -> Res<f32>;
    /// Reads a 64-bit floating point number.
    fn f64(&mut self, buf: &mut impl Buf) -> Res<f64>;

    /// Reads the NBT `end` tag, which indicates the end of a compound tag.
    fn end(&mut self, buf: &mut impl Buf) -> Res<()> {
        let t = self.u8(buf)?;
        if t != 0 {
            return Err(ErrorPath::new(ReadError::UnexpectedTag(
                "END (0x00)".to_string(),
                format!("{t:#04x}"),
            )));
        }
        Ok(())
    }

    /// Reads a variable-length string.
    fn string(&mut self, buf: &mut impl Buf) -> Res<String> {
        let len = self.i16(buf)?;
        if len < 0 {
            return Err(ErrorPath::new(ReadError::SeqLengthViolation(
                i16::MAX as usize,
                len as usize,
            )));
        }

        let mut str_buf = Vec::with_capacity(len as usize);
        for i in 0..len {
            str_buf.push(
                self.u8(buf)
                    .map_err(|err| err.prepend(PathPart::Element(i as usize)))?,
            );
        }

        String::from_utf8(str_buf).map_err(|err| ErrorPath::new(ReadError::from(err)))
    }

    /// Reads variable-length array of 8-bit unsigned integers.
    fn u8_vec(&mut self, buf: &mut impl Buf) -> Res<Vec<u8>> {
        let len = self.i32(buf)?;
        if len < 0 {
            return Err(ErrorPath::new(ReadError::SeqLengthViolation(
                i32::MAX as usize,
                len as usize,
            )));
        }

        let mut vec_buf = Vec::with_capacity(len as usize);
        for i in 0..len {
            vec_buf.push(
                self.u8(buf)
                    .map_err(|err| err.prepend(PathPart::Element(i as usize)))?,
            );
        }

        Ok(vec_buf)
    }

    /// Reads variable-length array of 32-bit signed integers.
    fn i32_vec(&mut self, buf: &mut impl Buf) -> Res<Vec<i32>> {
        let len = self.i32(buf)?;
        if len < 0 {
            return Err(ErrorPath::new(ReadError::SeqLengthViolation(
                i32::MAX as usize,
                len as usize,
            )));
        }

        let mut vec_buf = Vec::with_capacity(len as usize);
        for i in 0..len {
            vec_buf.push(
                self.i32(buf)
                    .map_err(|err| err.prepend(PathPart::Element(i as usize)))?,
            );
        }

        Ok(vec_buf)
    }

    /// Reads variable-length array of 64-bit signed integers.
    fn i64_vec(&mut self, buf: &mut impl Buf) -> Res<Vec<i64>> {
        let len = self.i32(buf)?;
        if len < 0 {
            return Err(ErrorPath::new(ReadError::SeqLengthViolation(
                i32::MAX as usize,
                len as usize,
            )));
        }

        let mut vec_buf = Vec::with_capacity(len as usize);
        for i in 0..len {
            vec_buf.push(
                self.i64(buf)
                    .map_err(|err| err.prepend(PathPart::Element(i as usize)))?,
            );
        }

        Ok(vec_buf)
    }
}
