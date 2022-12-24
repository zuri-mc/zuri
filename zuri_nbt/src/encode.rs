use bytes::{BufMut, BytesMut};

pub trait Writer {
    fn bool(&mut self, x: bool);
    fn u8(&mut self, x: u8);
    fn i16(&mut self, x: i16);
    fn i32(&mut self, x: i32);
    fn i64(&mut self, x: i64);
    fn f32(&mut self, x: f32);
    fn f64(&mut self, x: f64);

    fn end(&mut self) {
        self.u8(0);
    }

    fn string(&mut self, x: &str) {
        if x.len() > i16::MAX as usize {
            todo!();
        }

        self.i16(x.len() as i16);
        x.as_bytes().iter().for_each(|b| self.u8(*b));
    }

    fn u8_vec(&mut self, x: &Vec<u8>) {
        self.i32(x.len() as i32);
        x.iter().for_each(|v| self.u8(*v));
    }

    fn i32_vec(&mut self, x: &Vec<i32>) {
        self.i32(x.len() as i32);
        x.iter().for_each(|v| self.i32(*v));
    }

    fn i64_vec(&mut self, x: &Vec<i64>) {
        self.i32(x.len() as i32);
        x.iter().for_each(|v| self.i64(*v));
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
    fn bool(&mut self, x: bool) {
        self.buf.put_u8(x as u8);
    }

    fn u8(&mut self, x: u8) {
        self.buf.put_u8(x);
    }

    fn i16(&mut self, x: i16) {
        self.buf.put_i16_le(x);
    }

    fn i32(&mut self, x: i32) {
        self.buf.put_i32_le(x);
    }

    fn i64(&mut self, x: i64) {
        self.buf.put_i64_le(x);
    }

    fn f32(&mut self, x: f32) {
        self.buf.put_f32_le(x);
    }

    fn f64(&mut self, x: f64) {
        self.buf.put_f64_le(x);
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
    fn bool(&mut self, x: bool) {
        self.buf.put_u8(x as u8);
    }

    fn u8(&mut self, x: u8) {
        self.buf.put_u8(x);
    }

    fn i16(&mut self, x: i16) {
        self.buf.put_i16_le(x);
    }

    fn i32(&mut self, x: i32) {
        let mut u = (x as u32) << 1;
        if x < 0 {
            u = !u;
        }
        while u >= 0x80 {
            self.u8(u as u8 | 0x80);
            u >>= 7;
        }
        self.u8(u as u8);
    }

    fn i64(&mut self, x: i64) {
        let mut u = (x as u64) << 1;
        if x < 0 {
            u = !u;
        }
        while u >= 0x80 {
            self.u8(u as u8 | 0x80);
            u >>= 7;
        }
        self.u8(u as u8);
    }

    fn f32(&mut self, x: f32) {
        self.buf.put_f32_le(x);
    }

    fn f64(&mut self, x: f64) {
        self.buf.put_f64_le(x);
    }

    fn string(&mut self, x: &str) {
        if x.len() > i16::MAX as usize {
            todo!();
        }

        let mut l = x.len() as u32;
        while l >= 0x80 {
            self.u8(l as u8 | 0x80);
            l >>= 7;
        }
        self.u8(l as u8);
        x.as_bytes().iter().for_each(|b| self.u8(*b));
    }
}
