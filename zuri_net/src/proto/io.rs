use std::collections::{HashMap, VecDeque};

use bytes::{Buf, BufMut, Bytes, BytesMut};
use glam::{IVec2, IVec3, Vec2, Vec3};
use num_traits::{FromPrimitive, ToPrimitive};
use uuid::Uuid;

use zuri_nbt::encoding::NetworkLittleEndian;
use zuri_nbt::{decode, encode, NBTTag};

use crate::proto::types::entity_data::{EntityDataEntry, EntityDataType};

#[derive(Default)]
pub struct Writer {
    buf: BytesMut,
    shield_id: i32,
}

impl From<Writer> for BytesMut {
    fn from(value: Writer) -> Self {
        value.buf
    }
}

impl From<Writer> for Bytes {
    fn from(value: Writer) -> Self {
        value.buf.into()
    }
}

impl From<Writer> for Vec<u8> {
    fn from(value: Writer) -> Self {
        value.buf.into()
    }
}

impl From<Writer> for VecDeque<u8> {
    fn from(value: Writer) -> Self {
        <BytesMut as Into<Vec<u8>>>::into(value.buf).into()
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

    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
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

    pub fn nbt<T: encode::Writer + Sized>(&mut self, val: &NBTTag, mut writer: T) {
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

        let mut keys: Vec<u32> = x.keys().copied().collect();
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

impl<T: Writable, const N: usize> Writable for [T; N] {
    #[inline]
    fn write(&self, writer: &mut Writer) {
        self.iter().for_each(|i| i.write(writer));
    }
}

impl<T: Writable> Writable for Option<T> {
    #[inline]
    fn write(&self, writer: &mut Writer) {
        writer.optional(self);
    }
}

impl Writable for Vec3 {
    #[inline]
    fn write(&self, writer: &mut Writer) {
        writer.vec3(*self);
    }
}

impl Writable for Vec2 {
    #[inline]
    fn write(&self, writer: &mut Writer) {
        writer.vec2(*self);
    }
}

impl Writable for IVec2 {
    #[inline]
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.x);
        writer.var_i32(self.y);
    }
}

impl Writable for Uuid {
    #[inline]
    fn write(&self, writer: &mut Writer) {
        writer.uuid(*self);
    }
}

impl Writable for bool {
    #[inline]
    fn write(&self, writer: &mut Writer) {
        writer.u8(*self as u8);
    }
}

impl Writable for u8 {
    #[inline]
    fn write(&self, writer: &mut Writer) {
        writer.u8(*self);
    }
}

impl Writable for u16 {
    #[inline]
    fn write(&self, writer: &mut Writer) {
        writer.u16(*self);
    }
}

impl Writable for u32 {
    #[inline]
    fn write(&self, writer: &mut Writer) {
        writer.u32(*self);
    }
}

impl Writable for u64 {
    #[inline]
    fn write(&self, writer: &mut Writer) {
        writer.u64(*self);
    }
}

impl Writable for i8 {
    #[inline]
    fn write(&self, writer: &mut Writer) {
        writer.i8(*self);
    }
}

impl Writable for i16 {
    #[inline]
    fn write(&self, writer: &mut Writer) {
        writer.i16(*self);
    }
}

impl Writable for i32 {
    #[inline]
    fn write(&self, writer: &mut Writer) {
        writer.i32(*self);
    }
}

impl Writable for i64 {
    #[inline]
    fn write(&self, writer: &mut Writer) {
        writer.i64(*self);
    }
}

impl Writable for f32 {
    #[inline]
    fn write(&self, writer: &mut Writer) {
        writer.f32(*self);
    }
}

impl Writable for Bytes {
    #[inline]
    fn write(&self, writer: &mut Writer) {
        writer.byte_slice(self);
    }
}

impl Writable for String {
    #[inline]
    fn write(&self, writer: &mut Writer) {
        writer.string(self);
    }
}

#[derive(Default)]
pub struct Reader {
    buf: Bytes,
    shield_id: i32,
}

impl From<Reader> for Bytes {
    fn from(value: Reader) -> Self {
        value.buf
    }
}

impl Reader {
    pub fn from_buf(buf: Bytes, shield_id: i32) -> Self {
        Reader { buf, shield_id }
    }

    pub fn shield_id(&self) -> i32 {
        self.shield_id
    }

    pub fn len(&self) -> usize {
        self.buf.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }

    pub fn u8(&mut self) -> u8 {
        self.buf.get_u8()
    }

    pub fn i8(&mut self) -> i8 {
        self.buf.get_i8()
    }

    pub fn u16(&mut self) -> u16 {
        self.buf.get_u16_le()
    }

    pub fn i16(&mut self) -> i16 {
        self.buf.get_i16_le()
    }

    pub fn u32(&mut self) -> u32 {
        self.buf.get_u32_le()
    }

    pub fn i32(&mut self) -> i32 {
        self.buf.get_i32_le()
    }

    pub fn i32_be(&mut self) -> i32 {
        self.buf.get_i32()
    }

    pub fn u64(&mut self) -> u64 {
        self.buf.get_u64_le()
    }

    pub fn i64(&mut self) -> i64 {
        self.buf.get_i64_le()
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
                return if v & 1 != 0 { -x } else { x };
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
                return if v & 1 != 0 { -x } else { x };
            }
        }
        panic!("varint overflows integer");
    }

    pub fn f32(&mut self) -> f32 {
        self.buf.get_f32_le()
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

    pub fn nbt<T: decode::Reader + Sized>(&mut self, mut reader: T) -> NBTTag {
        NBTTag::read(&mut self.buf, &mut reader).unwrap()
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

impl Readable<Vec3> for Vec3 {
    #[inline]
    fn read(reader: &mut Reader) -> Vec3 {
        reader.vec3()
    }
}

impl Readable<Vec2> for Vec2 {
    #[inline]
    fn read(reader: &mut Reader) -> Vec2 {
        reader.vec2()
    }
}

impl Readable<IVec2> for IVec2 {
    #[inline]
    fn read(reader: &mut Reader) -> IVec2 {
        IVec2::new(reader.var_i32(), reader.var_i32())
    }
}

impl Readable<Uuid> for Uuid {
    #[inline]
    fn read(reader: &mut Reader) -> Uuid {
        reader.uuid()
    }
}

impl<T: Readable<T>, const N: usize> Readable<[T; N]> for [T; N] {
    #[inline]
    fn read(reader: &mut Reader) -> [T; N] {
        match (0..N)
            .map(|_| T::read(reader))
            .collect::<Vec<T>>()
            .try_into()
        {
            Ok(r) => r,
            _ => unreachable!(),
        }
    }
}

impl<T: Readable<T>> Readable<Option<T>> for Option<T> {
    #[inline]
    fn read(reader: &mut Reader) -> Option<T> {
        match reader.bool() {
            true => Some(T::read(reader)),
            false => None,
        }
    }
}

impl Readable<bool> for bool {
    #[inline]
    fn read(reader: &mut Reader) -> bool {
        reader.bool()
    }
}

impl Readable<u8> for u8 {
    #[inline]
    fn read(reader: &mut Reader) -> u8 {
        reader.u8()
    }
}

impl Readable<u16> for u16 {
    #[inline]
    fn read(reader: &mut Reader) -> u16 {
        reader.u16()
    }
}

impl Readable<u32> for u32 {
    #[inline]
    fn read(reader: &mut Reader) -> u32 {
        reader.u32()
    }
}

impl Readable<u64> for u64 {
    #[inline]
    fn read(reader: &mut Reader) -> u64 {
        reader.u64()
    }
}

impl Readable<i8> for i8 {
    #[inline]
    fn read(reader: &mut Reader) -> i8 {
        reader.i8()
    }
}

impl Readable<i16> for i16 {
    #[inline]
    fn read(reader: &mut Reader) -> i16 {
        reader.i16()
    }
}

impl Readable<i32> for i32 {
    #[inline]
    fn read(reader: &mut Reader) -> i32 {
        reader.i32()
    }
}

impl Readable<i64> for i64 {
    #[inline]
    fn read(reader: &mut Reader) -> i64 {
        reader.i64()
    }
}

impl Readable<f32> for f32 {
    #[inline]
    fn read(reader: &mut Reader) -> f32 {
        reader.f32()
    }
}

impl Readable<String> for String {
    #[inline]
    fn read(reader: &mut Reader) -> String {
        reader.string()
    }
}

impl Readable<Bytes> for Bytes {
    #[inline]
    fn read(reader: &mut Reader) -> Bytes {
        reader.byte_slice()
    }
}

/// A special trait to allow enum discriminants to be read with different integer types.
pub trait EnumWritable<D> {
    fn write(&self, writer: &mut Writer);
}

/// A special trait to allow enum discriminants to be written with different integer types.
pub trait EnumReadable<T, D> {
    fn read(reader: &mut Reader) -> T;
}

#[derive(Clone, Debug)]
pub struct NBT<E: decode::Reader + encode::Writer> {
    val: NBTTag,
    encoding: E,
}

impl<E: decode::Reader + encode::Writer + Default> NBT<E> {
    fn new(val: NBTTag) -> Self {
        Self {
            val,
            encoding: E::default(),
        }
    }
}

impl<E: decode::Reader + encode::Writer> NBT<E> {
    #![allow(unused)]
    fn new_with_encoder(val: NBTTag, encoder: E) -> Self {
        Self {
            val,
            encoding: encoder,
        }
    }
}

impl<E: decode::Reader + encode::Writer> From<NBT<E>> for NBTTag {
    fn from(value: NBT<E>) -> Self {
        value.val
    }
}

impl<E: decode::Reader + encode::Writer + Default> From<NBTTag> for NBT<E> {
    fn from(value: NBTTag) -> Self {
        NBT::new(value)
    }
}

impl<E: decode::Reader + encode::Writer + Clone> Writable for NBT<E> {
    #[inline]
    fn write(&self, writer: &mut Writer) {
        writer.nbt(&self.val, self.encoding.clone())
    }
}

impl<E: decode::Reader + encode::Writer + Default> Readable<NBT<E>> for NBT<E> {
    #[inline]
    fn read(reader: &mut Reader) -> NBT<E> {
        NBT::new(reader.nbt(E::default()))
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct UBlockPos {
    pub x: i32,
    pub y: u32,
    pub z: i32,
}

impl Writable for UBlockPos {
    #[inline]
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.x);
        writer.var_u32(self.y);
        writer.var_i32(self.z);
    }
}

impl Readable<UBlockPos> for UBlockPos {
    #[inline]
    fn read(reader: &mut Reader) -> UBlockPos {
        Self {
            x: reader.var_i32(),
            y: reader.var_u32(),
            z: reader.var_i32(),
        }
    }
}

impl From<UBlockPos> for IVec3 {
    fn from(value: UBlockPos) -> Self {
        IVec3::new(value.x, value.y as i32, value.z)
    }
}

impl From<IVec3> for UBlockPos {
    fn from(value: IVec3) -> Self {
        Self {
            x: value.x,
            y: value.y as u32,
            z: value.z,
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub struct BlockPos(pub IVec3);

impl Writable for BlockPos {
    #[inline]
    fn write(&self, writer: &mut Writer) {
        writer.block_pos(self.0)
    }
}

impl Readable<BlockPos> for BlockPos {
    #[inline]
    fn read(reader: &mut Reader) -> BlockPos {
        Self(reader.block_pos())
    }
}

impl From<BlockPos> for IVec3 {
    fn from(value: BlockPos) -> Self {
        value.0
    }
}

impl From<IVec3> for BlockPos {
    fn from(value: IVec3) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod tests {
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
        assert_eq!(
            reader.string_utf(),
            <&str as Into<String>>::into("This is a test!")
        );
        assert_eq!(reader.var_i32(), 243563456);
    }
}
