use bytes::{Buf, Bytes};

pub trait Reader {
    fn read_bool(&mut self) -> bool;
    fn read_u8(&mut self) -> u8;
    fn read_i16(&mut self) -> i16;
    fn read_i32(&mut self) -> i32;
    fn read_i64(&mut self) -> i64;
    fn read_f32(&mut self) -> f32;
    fn read_f64(&mut self) -> f64;

    fn read_end(&mut self) {
        assert_eq!(self.read_u8(), 0);
    }

    fn read_string(&mut self) -> String {
        let len = self.read_i16();
        if len < 0 {
            panic!();
        }
        let mut buf = Vec::with_capacity(len as usize);
        for _ in 0..len {
            buf.push(self.read_u8());
        }
        String::from_utf8(buf).unwrap()
    }

    fn read_u8_vec(&mut self) -> Vec<u8> {
        let len = self.read_i32();
        if len < 0 {
            panic!();
        }
        let mut buf = Vec::with_capacity(len as usize);
        for _ in 0..len {
            buf.push(self.read_u8());
        }
        buf
    }

    fn read_i32_vec(&mut self) -> Vec<i32> {
        let len = self.read_i32();
        if len < 0 {
            panic!();
        }
        let mut buf = Vec::with_capacity(len as usize);
        for _ in 0..len {
            buf.push(self.read_i32());
        }
        buf
    }

    fn read_i64_vec(&mut self) -> Vec<i64> {
        let len = self.read_i32();
        if len < 0 {
            panic!();
        }
        let mut buf = Vec::with_capacity(len as usize);
        for _ in 0..len {
            buf.push(self.read_i64());
        }
        buf
    }
}

#[derive(Default)]
pub struct LittleEndianReader {
    buf: Bytes
}

impl From<Bytes> for LittleEndianReader {
    fn from(value: Bytes) -> Self {
        Self {
            buf: value,
        }
    }
}

impl Reader for LittleEndianReader {
    fn read_bool(&mut self) -> bool {
        self.buf.get_u8() != 0
    }

    fn read_u8(&mut self) -> u8 {
        self.buf.get_u8()
    }

    fn read_i16(&mut self) -> i16 {
        self.buf.get_i16_le()
    }

    fn read_i32(&mut self) -> i32 {
        self.buf.get_i32_le()
    }

    fn read_i64(&mut self) -> i64 {
        self.buf.get_i64_le()
    }

    fn read_f32(&mut self) -> f32 {
        self.buf.get_f32_le()
    }

    fn read_f64(&mut self) -> f64 {
        self.buf.get_f64_le()
    }
}
