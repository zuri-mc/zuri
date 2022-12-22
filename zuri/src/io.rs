use std::collections::VecDeque;

use bytes::{
    Buf,
    BufMut,
    Bytes,
    BytesMut,
};

#[derive(Default)]
pub struct Writer {
    buf: BytesMut,
}

impl Into<BytesMut> for Writer {
    fn into(self) -> BytesMut {
        self.buf
    }
}

impl Into<Bytes> for Writer {
    fn into(self) -> Bytes {
        self.buf.into()
    }
}

impl Writer {
    pub fn write_u8(&mut self, x: u8) {
        self.buf.put_u8(x);
    }

    pub fn write_i8(&mut self, x: i8) {
        self.buf.put_i8(x);
    }

    pub fn write_u16(&mut self, x: u16) {
        self.buf.put_u16_le(x);
    }

    pub fn write_i16(&mut self, x: i16) {
        self.buf.put_i16_le(x);
    }

    pub fn write_u32(&mut self, x: u32) {
        self.buf.put_u32_le(x);
    }

    pub fn write_i32(&mut self, x: i32) {
        self.buf.put_i32_le(x);
    }

    pub fn write_be32(&mut self, x: i32) {
        self.buf.put_i32(x);
    }

    pub fn write_u64(&mut self, x: u64) {
        self.buf.put_u64_le(x);
    }

    pub fn write_i64(&mut self, x: i64) {
        self.buf.put_i64_le(x);
    }

    pub fn write_var_u32(&mut self, mut x: u32) {
        while x >= 0x80 {
            self.write_u8(x as u8 | 0x80);
            x >>= 7;
        }
        self.write_u8(x as u8);
    }

    pub fn write_var_i32(&mut self, x: i32) {
        let mut u = (x as u32) << 1;
        if x < 0 {
            u = !u;
        }
        while u >= 0x80 {
            self.write_u8(u as u8 | 0x80);
            u >>= 7;
        }
        self.write_u8(u as u8);
    }

    pub fn write_var_u64(&mut self, mut x: u64) {
        while x >= 0x80 {
            self.write_u8(x as u8 | 0x80);
            x >>= 7;
        }
        self.write_u8(x as u8);
    }

    pub fn write_var_i64(&mut self, x: i64) {
        let mut u = (x as u64) << 1;
        if x < 0 {
            u = !u;
        }
        while u >= 0x80 {
            self.write_u8(u as u8 | 0x80);
            u >>= 7;
        }
        self.write_u8(u as u8);
    }

    pub fn write_f32(&mut self, x: f32) {
        self.buf.put_f32_le(x)
    }

    pub fn write_bool(&mut self, x: bool) {
        self.write_u8(x as u8);
    }

    pub fn write_string(&mut self, x: String) {
        self.write_var_u32(x.len() as u32);
        self.buf.put_slice(x.as_bytes());
    }

    pub fn write_string_utf(&mut self, x: String) {
        self.write_i16(x.len() as i16);
        self.buf.put_slice(x.as_bytes());
    }

    pub fn write_slice(&mut self, x: &[u8]) {
        self.write_var_u32(x.len() as u32);
        self.buf.put_slice(x);
    }
}

#[derive(Default)]
pub struct Reader {
    buf: Bytes,
}

impl Into<Bytes> for Reader {
    fn into(self) -> Bytes {
        self.buf
    }
}

impl Reader {
    pub fn from_bytes(buf: Bytes) -> Self {
        Reader {
            buf,
        }
    }

    pub fn read_u8(&mut self) -> u8 {
        return self.buf.get_u8();
    }

    pub fn read_i8(&mut self) -> i8 {
        return self.buf.get_i8();
    }

    pub fn read_u16(&mut self) -> u16 {
        return self.buf.get_u16_le();
    }

    pub fn read_i16(&mut self) -> i16 {
        return self.buf.get_i16_le();
    }

    pub fn read_u32(&mut self) -> u32 {
        return self.buf.get_u32_le();
    }

    pub fn read_i32(&mut self) -> i32 {
        return self.buf.get_i32_le();
    }

    pub fn read_be32(&mut self) -> i32 {
        return self.buf.get_i32();
    }

    pub fn read_u64(&mut self) -> u64 {
        return self.buf.get_u64_le();
    }

    pub fn read_i64(&mut self) -> i64 {
        return self.buf.get_i64_le();
    }

    pub fn read_var_u32(&mut self) -> u32 {
        let mut v: u32 = 0;
        for i in (0..35).step_by(7) {
            let b = self.read_u8();

            v |= ((b & 0x7f) as u32) << i;
            if b & 0x80 == 0 {
                return v;
            }
        }
        panic!("varint overflows integer");
    }

    pub fn read_var_i32(&mut self) -> i32 {
        let mut v: u32 = 0;
        for i in (0..35).step_by(7) {
            let b = self.read_u8();

            v |= ((b & 0x7f) as u32) << i;
            if b & 0x80 == 0 {
                let x = (v >> 1) as i32;
                return if v & 1 != 0 {
                    -x
                } else {
                    x
                };
            }
        }
        panic!("varint overflows integer");
    }

    pub fn read_var_u64(&mut self) -> u64 {
        let mut v: u64 = 0;
        for i in (0..70).step_by(7) {
            let b = self.read_u8();

            v |= ((b & 0x7f) as u64) << i;
            if b & 0x80 == 0 {
                return v;
            }
        }
        panic!("varint overflows integer");
    }

    pub fn read_var_i64(&mut self) -> i64 {
        let mut v: u64 = 0;
        for i in (0..70).step_by(7) {
            let b = self.read_u8();

            v |= ((b & 0x7f) as u64) << i;
            if b & 0x80 == 0 {
                let x = (v >> 1) as i64;
                return if v & 1 != 0 {
                    -x
                } else {
                    x
                };
            }
        }
        panic!("varint overflows integer");
    }

    pub fn read_f32(&mut self) -> f32 {
        return self.buf.get_f32_le();
    }

    pub fn read_bool(&mut self) -> bool {
        return self.read_u8() != 0;
    }

    pub fn read_string(&mut self) -> String {
        let len = self.read_var_u32() as usize;
        let s = String::from_utf8(self.buf.slice(0..len).into()).expect("could not decode string");
        self.buf.advance(len);
        s
    }

    pub fn read_string_utf(&mut self) -> String {
        let len = self.read_i16() as usize;
        let s = String::from_utf8(self.buf.slice(0..len).into()).expect("could not decode string");
        self.buf.advance(len);
        s
    }

    pub fn read_slice(&mut self) -> Bytes {
        let len = self.read_var_u32() as usize;
        let b = self.buf.slice(0..len);
        self.buf.advance(len);
        b
    }
}

#[cfg(test)]
mod tests {
    use bytes::BytesMut;

    use crate::io::{Reader, Writer};

    #[test]
    fn test_read() {
        let mut buf = Writer::default();
        buf.write_bool(true);
        buf.write_i32(23974);
        buf.write_string_utf("This is a test!".into());
        buf.write_var_i32(243563456);

        let mut reader = Reader::from_bytes(buf.into());
        assert_eq!(reader.read_bool(), true);
        assert_eq!(reader.read_i32(), 23974);
        assert_eq!(reader.read_string_utf(), <&str as Into<String>>::into("This is a test!"));
        assert_eq!(reader.read_var_i32(), 243563456);
    }
}
