use crate::data::*;

use bytes::{
    Buf,
    BufMut,
    Bytes,
    BytesMut,
};
use glam::{Vec2, Vec3};
use uuid::Uuid;

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

    pub fn string(&mut self, x: &str) {
        self.var_u32(x.len() as u32);
        self.buf.put_slice(x.as_bytes());
    }

    pub fn string_utf(&mut self, x: &str) {
        self.i16(x.len() as i16);
        self.buf.put_slice(x.as_bytes());
    }

    pub fn bytes(&mut self, x: &[u8]) {
        self.buf.put_slice(x);
    }

    pub fn write_slice(&mut self, x: &[u8]) {
        self.write_var_u32(x.len() as u32);
        self.buf.put_slice(x);
    }

    pub fn block_pos(&mut self, x: BlockPos) {
        self.i32(x.x);
        self.i32(x.y);
        self.i32(x.z);
    }

    pub fn u_block_pos(&mut self, x: BlockPos) {
        self.i32(x.x);
        self.u32(x.y as u32);
        self.i32(x.z);
    }

    pub fn vec2(&mut self, x: Vec2) {
        self.f32(x.x);
        self.f32(x.y);
    }

    pub fn vec3(&mut self, x: Vec3) {
        self.f32(x.x);
        self.f32(x.y);
        self.f32(x.z);
    }

    pub fn uuid(&mut self, x: Uuid) {
        self.buf.put(x.to_bytes_le());
    }

    pub fn optional<T>(&mut self, x: &Option<T>, f: fn(&T)) {
        self.bool(x.is_some());
        if let Some(x) = x {
            f(x);
        }
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

    pub fn u8(&mut self) -> u8 {
        return self.buf.get_u8();
    }

    pub fn i8(&mut self) -> i8 {
        return self.buf.get_i8();
    }

    pub fn u16(&mut self) -> u16 {
        return self.buf.get_u16_le();
    }

    pub fn i16(&mut self) -> i16 {
        return self.buf.get_i16_le();
    }

    pub fn u32(&mut self) -> u32 {
        return self.buf.get_u32_le();
    }

    pub fn i32(&mut self) -> i32 {
        return self.buf.get_i32_le();
    }

    pub fn i32_be(&mut self) -> i32 {
        return self.buf.get_i32();
    }

    pub fn u64(&mut self) -> u64 {
        return self.buf.get_u64_le();
    }

    pub fn i64(&mut self) -> i64 {
        return self.buf.get_i64_le();
    }

    pub fn var_u32(&mut self) -> u32 {
        let mut v: u32 = 0;
        for i in (0..35).step_by(7) {
            let b = self.u8();

            v |= ((b & 0x7f) as u32) << i;
            if b & 0x80 == 0 {
                return v;
            }
        }
        panic!("varint overflows integer");
    }

    pub fn var_i32(&mut self) -> i32 {
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

    pub fn var_u64(&mut self) -> u64 {
        let mut v: u64 = 0;
        for i in (0..70).step_by(7) {
            let b = self.u8();

            v |= ((b & 0x7f) as u64) << i;
            if b & 0x80 == 0 {
                return v;
            }
        }
        panic!("varint overflows integer");
    }

    pub fn var_i64(&mut self) -> i64 {
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

    pub fn f32(&mut self) -> f32 {
        return self.buf.get_f32_le();
    }

    pub fn byte_f32(&mut self) -> f32 {
        return (self.u8() as f32) * (360. / 256.);
    }

    pub fn bool(&mut self) -> bool {
        return self.u8() != 0;
    }

    pub fn string(&mut self) -> String {
        let len = self.var_u32() as usize;
        let s = String::from_utf8(self.buf.slice(0..len).into()).expect("could not decode string");
        self.buf.advance(len);
        s
    }

    pub fn string_utf(&mut self) -> String {
        let len = self.i16() as usize;
        let s = String::from_utf8(self.buf.slice(0..len).into()).expect("could not decode string");
        self.buf.advance(len);
        s
    }

    pub fn bytes(&mut self) -> Bytes {
        let len = self.buf.remaining();
        let b = self.buf.slice(0..len);
        self.buf.advance(len);
        b
    }

    pub fn read_slice(&mut self) -> Bytes {
        let len = self.read_var_u32() as usize;
        let b = self.buf.slice(0..len);
        self.buf.advance(len);
        b
    }

    pub fn block_pos(&mut self) -> BlockPos {
        BlockPos {
            x: self.i32(),
            y: self.i32(),
            z: self.i32(),
        }
    }

    pub fn u_block_pos(&mut self) -> BlockPos {
        BlockPos {
            x: self.i32(),
            y: self.u32() as i32,
            z: self.i32(),
        }
    }

    pub fn vec2(&mut self) -> Vec2 {
        Vec2 {
            x: self.f32(),
            y: self.f32(),
        }
    }

    pub fn vec3(&mut self) -> Vec3 {
        Vec3 {
            x: self.f32(),
            y: self.f32(),
            z: self.f32(),
        }
    }

    pub fn uuid(&mut self) -> Uuid {
        let b = self.buf.slice(0..16);
        self.buf.advance(16);
        Uuid::from_slice_le(&b).unwrap()
    }

    pub fn optional<T>(&mut self, f: fn() -> T) -> Option<T> {
        if self.bool() {
            Some(f())
        } else {
            None
        }
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
        assert_eq!(reader.bool(), true);
        assert_eq!(reader.i32(), 23974);
        assert_eq!(reader.string_utf(), <&str as Into<String>>::into("This is a test!"));
        assert_eq!(reader.var_i32(), 243563456);
    }
}
