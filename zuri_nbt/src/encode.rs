use bytes::{BufMut, BytesMut};
use crate::err::{NbtError, Res};

pub trait Writer {
    fn bool(&mut self, x: bool) -> Res<()>;
    fn u8(&mut self, x: u8) -> Res<()>;
    fn i16(&mut self, x: i16) -> Res<()>;
    fn i32(&mut self, x: i32) -> Res<()>;
    fn i64(&mut self, x: i64) -> Res<()>;
    fn f32(&mut self, x: f32) -> Res<()>;
    fn f64(&mut self, x: f64) -> Res<()>;

    fn end(&mut self) -> Res<()> {
        self.u8(0)
    }

    fn string(&mut self, x: &str) -> Res<()> {
        if x.len() > i16::MAX as usize {
            return Err(NbtError::ParseError("string too large".to_string()));
        }

        self.i16(x.len() as i16)?;
        for b in x.as_bytes() {
            self.u8(*b)?;
        }
        Ok(())
    }

    fn u8_vec(&mut self, x: &Vec<u8>) -> Res<()> {
        self.i32(x.len() as i32)?;
        for v in x {
            self.u8(*v)?;
        }
        Ok(())
    }

    fn i32_vec(&mut self, x: &Vec<i32>) -> Res<()> {
        self.i32(x.len() as i32)?;
        for v in x {
            self.i32(*v)?;
        }
        Ok(())
    }

    fn i64_vec(&mut self, x: &Vec<i64>) -> Res<()> {
        self.i32(x.len() as i32)?;
        for v in x {
            self.i64(*v)?;
        }
        Ok(())
    }
}

#[derive(Default)]
pub struct LittleEndianWriter {
    buf: BytesMut,
}

impl Into<BytesMut> for LittleEndianWriter {
    fn into(self) -> BytesMut {
        self.buf
    }
}

impl Writer for LittleEndianWriter {
    fn bool(&mut self, x: bool) -> Res<()> {
        self.buf.put_u8(x as u8);
        Ok(())
    }

    fn u8(&mut self, x: u8) -> Res<()> {
        self.buf.put_u8(x);
        Ok(())
    }

    fn i16(&mut self, x: i16) -> Res<()> {
        self.buf.put_i16_le(x);
        Ok(())
    }

    fn i32(&mut self, x: i32) -> Res<()> {
        self.buf.put_i32_le(x);
        Ok(())
    }

    fn i64(&mut self, x: i64) -> Res<()> {
        self.buf.put_i64_le(x);
        Ok(())
    }

    fn f32(&mut self, x: f32) -> Res<()> {
        self.buf.put_f32_le(x);
        Ok(())
    }

    fn f64(&mut self, x: f64) -> Res<()> {
        self.buf.put_f64_le(x);
        Ok(())
    }
}

#[derive(Default)]
pub struct NetworkLittleEndianWriter {
    buf: BytesMut,
}

impl Into<BytesMut> for NetworkLittleEndianWriter {
    fn into(self) -> BytesMut {
        self.buf
    }
}

impl Writer for NetworkLittleEndianWriter {
    fn bool(&mut self, x: bool) -> Res<()> {
        self.buf.put_u8(x as u8);
        Ok(())
    }

    fn u8(&mut self, x: u8) -> Res<()> {
        self.buf.put_u8(x);
        Ok(())
    }

    fn i16(&mut self, x: i16) -> Res<()> {
        self.buf.put_i16_le(x);
        Ok(())
    }

    fn i32(&mut self, x: i32) -> Res<()> {
        let mut u = (x as u32) << 1;
        if x < 0 {
            u = !u;
        }
        while u >= 0x80 {
            self.u8(u as u8 | 0x80)?;
            u >>= 7;
        }
        self.u8(u as u8)?;
        Ok(())
    }

    fn i64(&mut self, x: i64) -> Res<()> {
        let mut u = (x as u64) << 1;
        if x < 0 {
            u = !u;
        }
        while u >= 0x80 {
            self.u8(u as u8 | 0x80)?;
            u >>= 7;
        }
        self.u8(u as u8)?;
        Ok(())
    }

    fn f32(&mut self, x: f32) -> Res<()> {
        self.buf.put_f32_le(x);
        Ok(())
    }

    fn f64(&mut self, x: f64) -> Res<()> {
        self.buf.put_f64_le(x);
        Ok(())
    }

    fn string(&mut self, x: &str) -> Res<()> {
        if x.len() > i16::MAX as usize {
            return Err(NbtError::ParseError("string too large".to_string()));
        }

        let mut l = x.len() as u32;
        while l >= 0x80 {
            self.u8(l as u8 | 0x80)?;
            l >>= 7;
        }
        self.u8(l as u8)?;
        for b in x.as_bytes() {
            self.u8(*b)?;
        }
        Ok(())
    }
}
