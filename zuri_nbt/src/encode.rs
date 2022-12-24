use bytes::{BufMut, BytesMut};

pub trait Writer {
    fn write_bool(&mut self, x: bool);
    fn write_u8(&mut self, x: u8);
    fn write_i16(&mut self, x: i16);
    fn write_i32(&mut self, x: i32);
    fn write_i64(&mut self, x: i64);
    fn write_f32(&mut self, x: f32);
    fn write_f64(&mut self, x: f64);

    fn write_end(&mut self) {
        self.write_u8(0);
    }

    fn write_string(&mut self, x: &str) {
        if x.len() > i16::MAX as usize {
            todo!();
        }

        self.write_i16(x.len() as i16);
        x.as_bytes().iter().for_each(|b| self.write_u8(*b));
    }

    fn write_u8_vec(&mut self, x: &Vec<u8>) {
        self.write_i32(x.len() as i32);
        x.iter().for_each(|v| self.write_u8(*v));
    }

    fn write_i32_vec(&mut self, x: &Vec<i32>) {
        self.write_i32(x.len() as i32);
        x.iter().for_each(|v| self.write_i32(*v));
    }

    fn write_i64_vec(&mut self, x: &Vec<i64>) {
        self.write_i32(x.len() as i32);
        x.iter().for_each(|v| self.write_i64(*v));
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
    fn write_bool(&mut self, x: bool) {
        self.buf.put_u8(x as u8);
    }

    fn write_u8(&mut self, x: u8) {
        self.buf.put_u8(x);
    }

    fn write_i16(&mut self, x: i16) {
        self.buf.put_i16_le(x);
    }

    fn write_i32(&mut self, x: i32) {
        self.buf.put_i32_le(x);
    }

    fn write_i64(&mut self, x: i64) {
        self.buf.put_i64_le(x);
    }

    fn write_f32(&mut self, x: f32) {
        self.buf.put_f32_le(x);
    }

    fn write_f64(&mut self, x: f64) {
        self.buf.put_f64_le(x);
    }
}
