use bytes::{Buf, Bytes};

pub trait Reader {
    fn bool(&mut self) -> bool;
    fn u8(&mut self) -> u8;
    fn i16(&mut self) -> i16;
    fn i32(&mut self) -> i32;
    fn i64(&mut self) -> i64;
    fn f32(&mut self) -> f32;
    fn f64(&mut self) -> f64;

    fn end(&mut self) {
        assert_eq!(self.u8(), 0);
    }

    fn string(&mut self) -> String {
        let len = self.i16();
        if len < 0 {
            panic!();
        }

        let mut buf = Vec::with_capacity(len as usize);
        (0..len).for_each(|_| buf.push(self.u8()));

        String::from_utf8(buf).unwrap()
    }

    fn u8_vec(&mut self) -> Vec<u8> {
        let len = self.i32();
        if len < 0 {
            panic!();
        }

        let mut buf = Vec::with_capacity(len as usize);
        (0..len).for_each(|_| buf.push(self.u8()));

        buf
    }

    fn i32_vec(&mut self) -> Vec<i32> {
        let len = self.i32();
        if len < 0 {
            panic!();
        }

        let mut buf = Vec::with_capacity(len as usize);
        (0..len).for_each(|_| buf.push(self.i32()));

        buf
    }

    fn i64_vec(&mut self) -> Vec<i64> {
        let len = self.i32();
        if len < 0 {
            panic!();
        }

        let mut buf = Vec::with_capacity(len as usize);
        (0..len).for_each(|_| buf.push(self.i64()));

        buf
    }
}

#[derive(Default)]
pub struct LittleEndianReader {
    buf: Bytes,
}

impl From<Bytes> for LittleEndianReader {
    fn from(value: Bytes) -> Self {
        Self { buf: value }
    }
}

impl Reader for LittleEndianReader {
    fn bool(&mut self) -> bool {
        self.buf.get_u8() != 0
    }

    fn u8(&mut self) -> u8 {
        self.buf.get_u8()
    }

    fn i16(&mut self) -> i16 {
        self.buf.get_i16_le()
    }

    fn i32(&mut self) -> i32 {
        self.buf.get_i32_le()
    }

    fn i64(&mut self) -> i64 {
        self.buf.get_i64_le()
    }

    fn f32(&mut self) -> f32 {
        self.buf.get_f32_le()
    }

    fn f64(&mut self) -> f64 {
        self.buf.get_f64_le()
    }
}

#[derive(Default)]
pub struct NetworkLittleEndianReader {
    buf: Bytes,
}

impl From<Bytes> for NetworkLittleEndianReader {
    fn from(value: Bytes) -> Self {
        Self { buf: value }
    }
}

impl Reader for NetworkLittleEndianReader {
    fn bool(&mut self) -> bool {
        self.buf.get_u8() != 0
    }

    fn u8(&mut self) -> u8 {
        self.buf.get_u8()
    }

    fn i16(&mut self) -> i16 {
        self.buf.get_i16_le()
    }

    fn i32(&mut self) -> i32 {
        let mut v: u32 = 0;
        for i in (0..35).step_by(7) {
            let b = self.u8();

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

    fn i64(&mut self) -> i64 {
        let mut v: u64 = 0;
        for i in (0..70).step_by(7) {
            let b = self.u8();

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

    fn f32(&mut self) -> f32 {
        self.buf.get_f32_le()
    }

    fn f64(&mut self) -> f64 {
        self.buf.get_f64_le()
    }

    fn string(&mut self) -> String {
        let len = 'var_len: {
            let mut v: u32 = 0;
            for i in (0..35).step_by(7) {
                let b = self.u8();

                v |= ((b & 0x7f) as u32) << i;
                if b & 0x80 == 0 {
                    break 'var_len v;
                }
            }
            panic!("varint overflows integer");
        };

        let mut buf = Vec::with_capacity(len as usize);
        (0..len).for_each(|_| buf.push(self.u8()));

        String::from_utf8(buf).unwrap()
    }
}
