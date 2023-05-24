//! See [Reader].
use crate::err::{NbtError, Res};
use bytes::Buf;

/// A trait that can be implemented to alter how basic NBT types are read.
pub trait Reader {
    /// Reads an 8-bit unsigned integer.
    fn u8(&mut self, buf: &mut impl Buf) -> Res<u8> {
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
            return Err(NbtError::ParseError(format!("expected TAG_end, got {}", t)));
        }
        Ok(())
    }

    /// Reads a variable-length string.
    fn string(&mut self, buf: &mut impl Buf) -> Res<String> {
        let len = self.i16(buf)?;
        if len < 0 {
            return Err(NbtError::ParseError(
                "string length must be greater than 0".to_string(),
            ));
        }

        let mut str_buf = Vec::with_capacity(len as usize);
        for _ in 0..len {
            str_buf.push(self.u8(buf)?);
        }

        Ok(String::from_utf8(str_buf)?)
    }

    /// Reads variable-length array of 8-bit unsigned integers.
    fn u8_vec(&mut self, buf: &mut impl Buf) -> Res<Vec<u8>> {
        let len = self.i32(buf)?;
        if len < 0 {
            return Err(NbtError::ParseError(
                "vec length must be greater than 0".to_string(),
            ));
        }

        let mut vec_buf = Vec::with_capacity(len as usize);
        for _ in 0..len {
            vec_buf.push(self.u8(buf)?);
        }

        Ok(vec_buf)
    }

    /// Reads variable-length array of 32-bit signed integers.
    fn i32_vec(&mut self, buf: &mut impl Buf) -> Res<Vec<i32>> {
        let len = self.i32(buf)?;
        if len < 0 {
            return Err(NbtError::ParseError(
                "vec length must be greater than 0".to_string(),
            ));
        }

        let mut vec_buf = Vec::with_capacity(len as usize);
        for _ in 0..len {
            vec_buf.push(self.i32(buf)?);
        }

        Ok(vec_buf)
    }

    /// Reads variable-length array of 64-bit signed integers.
    fn i64_vec(&mut self, buf: &mut impl Buf) -> Res<Vec<i64>> {
        let len = self.i32(buf)?;
        if len < 0 {
            return Err(NbtError::ParseError(
                "vec length must be greater than 0".to_string(),
            ));
        }

        let mut vec_buf = Vec::with_capacity(len as usize);
        for _ in 0..len {
            vec_buf.push(self.i64(buf)?);
        }

        Ok(vec_buf)
    }
}
