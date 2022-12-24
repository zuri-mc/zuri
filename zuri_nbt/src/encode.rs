use bytes::BufMut;
use crate::err::{NbtError, Res};

pub trait Writer {
    fn write_bool(&mut self, buf: &mut impl BufMut, x: bool) -> Res<()> {
        buf.put_u8(x as u8);
        Ok(())
    }
    fn write_u8(&mut self, buf: &mut impl BufMut, x: u8) -> Res<()> {
        buf.put_u8(x);
        Ok(())
    }
    fn write_i16(&mut self, buf: &mut impl BufMut, x: i16) -> Res<()>;
    fn write_i32(&mut self, buf: &mut impl BufMut, x: i32) -> Res<()>;
    fn write_i64(&mut self, buf: &mut impl BufMut, x: i64) -> Res<()>;
    fn write_f32(&mut self, buf: &mut impl BufMut, x: f32) -> Res<()>;
    fn write_f64(&mut self, buf: &mut impl BufMut, x: f64) -> Res<()>;

    fn write_end(&mut self, buf: &mut impl BufMut) -> Res<()> {
        buf.put_u8(0);
        Ok(())
    }

    fn write_string(&mut self, buf: &mut impl BufMut, x: &str) -> Res<()> {
        if x.len() > i16::MAX as usize {
            return Err(NbtError::ParseError("string too large".to_string()));
        }

        self.write_i16(buf, x.len() as i16)?;
        for b in x.as_bytes() {
            self.write_u8(buf, *b)?;
        }
        Ok(())
    }

    fn write_u8_vec(&mut self, buf: &mut impl BufMut, x: &Vec<u8>) -> Res<()> {
        self.write_i32(buf, x.len() as i32)?;
        for v in x {
            self.write_u8(buf, *v)?;
        }
        Ok(())
    }

    fn write_i32_vec(&mut self, buf: &mut impl BufMut, x: &Vec<i32>) -> Res<()> {
        self.write_i32(buf, x.len() as i32)?;
        for v in x {
            self.write_i32(buf, *v)?;
        }
        Ok(())
    }

    fn write_i64_vec(&mut self, buf: &mut impl BufMut, x: &Vec<i64>) -> Res<()> {
        self.write_i32(buf, x.len() as i32)?;
        for v in x {
            self.write_i64(buf, *v)?;
        }
        Ok(())
    }
}