use crate::err::{NbtError, Res};
use bytes::Buf;

pub trait Reader {
    fn bool(&mut self, buf: &mut impl Buf) -> Res<bool> {
        Ok(buf.get_u8() != 0)
    }
    fn u8(&mut self, buf: &mut impl Buf) -> Res<u8> {
        Ok(buf.get_u8())
    }
    fn i16(&mut self, buf: &mut impl Buf) -> Res<i16>;
    fn i32(&mut self, buf: &mut impl Buf) -> Res<i32>;
    fn i64(&mut self, buf: &mut impl Buf) -> Res<i64>;
    fn f32(&mut self, buf: &mut impl Buf) -> Res<f32>;
    fn f64(&mut self, buf: &mut impl Buf) -> Res<f64>;

    fn end(&mut self, buf: &mut impl Buf) -> Res<()> {
        let t = self.u8(buf)?;
        if t != 0 {
            return Err(NbtError::ParseError(format!("expected TAG_end, got {}", t)));
        }
        Ok(())
    }

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
