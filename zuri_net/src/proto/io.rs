use uuid::Uuid;
use std::collections::{HashMap, VecDeque};
use zuri_nbt::encoding::NetworkLittleEndian;
use num_traits::{FromPrimitive, ToPrimitive};
use crate::proto::types::entity_data::{EntityDataEntry, EntityDataType};

use glam::{
    IVec3,
    Vec2,
    Vec3,
};
use zuri_nbt::{
    Value,
    decode,
    encode,
};
use bytes::{
    Buf,
    BufMut,
    Bytes,
    BytesMut,
};

#[derive(Default)]
pub struct Writer {
    buf: BytesMut,
    shield_id: i32,
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

impl Into<Vec<u8>> for Writer {
    fn into(self) -> Vec<u8> {
        self.buf.into()
    }
}

impl Into<VecDeque<u8>> for Writer {
    fn into(self) -> VecDeque<u8> {
        <BytesMut as Into<Vec<u8>>>::into(self.buf).into()
    }
}

impl Writer {
    pub fn new(shield_id: i32) -> Writer {
        Writer {
            shield_id,
            ..Default::default()
        }
    }

    pub fn shield_id(&self) -> i32 {
        self.shield_id
    }

    pub fn len(&self) -> usize {
        self.buf.len()
    }

    pub fn u8(&mut self, x: u8) {
        self.buf.put_u8(x);
    }

    pub fn i8(&mut self, x: i8) {
        self.buf.put_i8(x);
    }

    pub fn u16(&mut self, x: u16) {
        self.buf.put_u16_le(x);
    }

    pub fn i16(&mut self, x: i16) {
        self.buf.put_i16_le(x);
    }

    pub fn u32(&mut self, x: u32) {
        self.buf.put_u32_le(x);
    }

    pub fn i32(&mut self, x: i32) {
        self.buf.put_i32_le(x);
    }

    pub fn i32_be(&mut self, x: i32) {
        self.buf.put_i32(x);
    }

    pub fn u64(&mut self, x: u64) {
        self.buf.put_u64_le(x);
    }

    pub fn i64(&mut self, x: i64) {
        self.buf.put_i64_le(x);
    }

    pub fn var_u32(&mut self, mut x: u32) {
        while x >= 0x80 {
            self.u8(x as u8 | 0x80);
            x >>= 7;
        }
        self.u8(x as u8);
    }

    pub fn var_i32(&mut self, x: i32) {
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

    pub fn var_u64(&mut self, mut x: u64) {
        while x >= 0x80 {
            self.u8(x as u8 | 0x80);
            x >>= 7;
        }
        self.u8(x as u8);
    }

    pub fn var_i64(&mut self, x: i64) {
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

    pub fn f32(&mut self, x: f32) {
        self.buf.put_f32_le(x)
    }

    pub fn byte_f32(&mut self, x: f32) {
        self.u8((x / (360.0 / 256.0)) as u8)
    }

    pub fn bool(&mut self, x: bool) {
        self.u8(x as u8);
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

    pub fn byte_slice(&mut self, x: &[u8]) {
        self.var_u32(x.len() as u32);
        self.buf.put_slice(x);
    }

    pub fn block_pos(&mut self, x: IVec3) {
        self.var_i32(x.x);
        self.var_i32(x.y);
        self.var_i32(x.z);
    }

    pub fn u_block_pos(&mut self, x: IVec3) {
        self.var_i32(x.x);
        self.var_u32(x.y as u32);
        self.var_i32(x.z);
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
        self.buf.put(x.to_bytes_le().as_ref());
    }

    pub fn nbt<T: encode::Writer + Sized>(&mut self, val: &Value, mut writer: T) {
        val.write(&mut self.buf, &mut writer).unwrap();
    }

    pub fn optional(&mut self, x: &Option<impl Writable>) {
        self.bool(x.is_some());
        if let Some(x) = x {
            x.write(self);
        }
    }

    pub fn entity_metadata(&mut self, x: &HashMap<u32, EntityDataEntry>) {
        self.var_u32(x.len() as u32);

        let mut keys: Vec<u32> = x.keys().map(|k| *k).collect();
        keys.sort();

        for key in keys {
            self.var_u32(key);
            match x.get(&key).unwrap() {
                EntityDataEntry::U8(v) => {
                    self.var_u32(EntityDataType::U8.to_u32().unwrap());
                    self.u8(*v);
                }
                EntityDataEntry::I16(v) => {
                    self.var_u32(EntityDataType::I16.to_u32().unwrap());
                    self.i16(*v);
                }
                EntityDataEntry::I32(v) => {
                    self.var_u32(EntityDataType::I32.to_u32().unwrap());
                    self.var_i32(*v);
                }
                EntityDataEntry::F32(v) => {
                    self.var_u32(EntityDataType::F32.to_u32().unwrap());
                    self.f32(*v);
                }
                EntityDataEntry::String(v) => {
                    self.var_u32(EntityDataType::String.to_u32().unwrap());
                    self.string(v);
                }
                EntityDataEntry::NBT(v) => {
                    self.var_u32(EntityDataType::NBT.to_u32().unwrap());
                    self.nbt(v, NetworkLittleEndian);
                }
                EntityDataEntry::BlockPos(v) => {
                    self.var_u32(EntityDataType::BlockPos.to_u32().unwrap());
                    self.block_pos(*v);
                }
                EntityDataEntry::I64(v) => {
                    self.var_u32(EntityDataType::I64.to_u32().unwrap());
                    self.var_i64(*v);
                }
                EntityDataEntry::Vec3(v) => {
                    self.var_u32(EntityDataType::Vec3.to_u32().unwrap());
                    self.vec3(*v);
                }
            }
        }
    }
}

pub trait Writable {
    fn write(&self, writer: &mut Writer);
}

impl Writable for bool {
    fn write(&self, writer: &mut Writer) {
        writer.u8(*self as u8);
    }
}

impl Writable for u8 {
    fn write(&self, writer: &mut Writer) {
        writer.u8(*self);
    }
}

impl Writable for Bytes {
    fn write(&self, writer: &mut Writer) {
        writer.byte_slice(self);
    }
}

impl Writable for String {
    fn write(&self, writer: &mut Writer) {
        writer.string(self);
    }
}

#[derive(Default)]
pub struct Reader {
    buf: Bytes,
    shield_id: i32,
}

impl Into<Bytes> for Reader {
    fn into(self) -> Bytes {
        self.buf
    }
}

impl Reader {
    pub fn from_buf(buf: Bytes, shield_id: i32) -> Self {
        Reader {
            buf,
            shield_id,
        }
    }

    pub fn shield_id(&self) -> i32 {
        self.shield_id
    }

    pub fn len(&self) -> usize {
        self.buf.len()
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
        (self.u8() as f32) * (360. / 256.)
    }

    pub fn bool(&mut self) -> bool {
        self.u8() != 0
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

    pub fn byte_slice(&mut self) -> Bytes {
        let len = self.var_u32() as usize;
        let b = self.buf.slice(0..len);
        self.buf.advance(len);
        b
    }

    pub fn block_pos(&mut self) -> IVec3 {
        IVec3 {
            x: self.var_i32(),
            y: self.var_i32(),
            z: self.var_i32(),
        }
    }

    pub fn u_block_pos(&mut self) -> IVec3 {
        IVec3 {
            x: self.var_i32(),
            y: self.var_u32() as i32,
            z: self.var_i32(),
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

    pub fn nbt<T: decode::Reader + Sized>(&mut self, mut reader: T) -> Value {
        Value::read(&mut self.buf, &mut reader).unwrap()
    }

    pub fn optional<T: Readable<T>>(&mut self) -> Option<T> {
        if self.bool() {
            Some(T::read(self))
        } else {
            None
        }
    }

    pub fn entity_metadata(&mut self) -> HashMap<u32, EntityDataEntry> {
        let mut metadata = HashMap::new();
        for _ in 0..self.var_u32() {
            let key = self.var_u32();
            match EntityDataType::from_u32(self.var_u32()).unwrap() {
                EntityDataType::U8 => {
                    metadata.insert(key, EntityDataEntry::U8(self.u8()));
                }
                EntityDataType::I16 => {
                    metadata.insert(key, EntityDataEntry::I16(self.i16()));
                }
                EntityDataType::I32 => {
                    metadata.insert(key, EntityDataEntry::I32(self.var_i32()));
                }
                EntityDataType::F32 => {
                    metadata.insert(key, EntityDataEntry::F32(self.f32()));
                }
                EntityDataType::String => {
                    metadata.insert(key, EntityDataEntry::String(self.string()));
                }
                EntityDataType::NBT => {
                    metadata.insert(key, EntityDataEntry::NBT(self.nbt(NetworkLittleEndian)));
                }
                EntityDataType::BlockPos => {
                    metadata.insert(key, EntityDataEntry::BlockPos(self.block_pos()));
                }
                EntityDataType::I64 => {
                    metadata.insert(key, EntityDataEntry::I64(self.var_i64()));
                }
                EntityDataType::Vec3 => {
                    metadata.insert(key, EntityDataEntry::Vec3(self.vec3()));
                }
            }
        }
        metadata
    }
}

pub trait Readable<T> {
    fn read(reader: &mut Reader) -> T;
}

impl Readable<bool> for bool {
    fn read(reader: &mut Reader) -> bool {
        reader.bool()
    }
}

impl Readable<u8> for u8 {
    fn read(reader: &mut Reader) -> u8 {
        reader.u8()
    }
}

impl Readable<String> for String {
    fn read(reader: &mut Reader) -> String {
        reader.string()
    }
}

impl Readable<Bytes> for Bytes {
    fn read(reader: &mut Reader) -> Bytes {
        reader.bytes()
    }
}

#[cfg(test)]
mod tests {
    use bytes::BytesMut;

    use crate::proto::io::{Reader, Writer};

    #[test]
    fn test_read() {
        let mut buf = Writer::default();
        buf.bool(true);
        buf.i32(23974);
        buf.string_utf("This is a test!".into());
        buf.var_i32(243563456);

        let mut reader = Reader::from_buf(buf.into(), 0);
        assert_eq!(reader.bool(), true);
        assert_eq!(reader.i32(), 23974);
        assert_eq!(reader.string_utf(), <&str as Into<String>>::into("This is a test!"));
        assert_eq!(reader.var_i32(), 243563456);
    }
}
