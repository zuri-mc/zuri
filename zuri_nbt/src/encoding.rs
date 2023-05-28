//! Contains all the standard NBT encodings.
//!
//! These include:
//!  - [BigEndian]
//!  - [LittleEndian]
//!  - [NetworkLittleEndian]
use crate::decode::Reader;
use crate::encode::Writer;
use crate::err::{NbtError, Res};
use bytes::{Buf, BufMut};

/// An NBT encoding that encodes all basic types using big endian encoding.
///
/// This format is most commonly used in Minecraft: Java Edition.
#[derive(Debug, Default, Clone)]
pub struct BigEndian;

/// An NBT encoding that encodes all basic types using little endian encoding.
///
/// This format is most commonly used in Minecraft: Bedrock Edition, and more specifically in
/// Bedrock Edition world saves.
///
/// It is not to be confused with the [NetworkLittleEndian] encoding.
#[derive(Debug, Default, Clone)]
pub struct LittleEndian;

/// An NBT encoding that encodes certain integer types using variable-length encoding, while using
/// fixed-size little endian encoding for all other basic types.
///
/// This format is most commonly used for nbt sent in Minecraft: Bedrock Edition's protocol.
#[derive(Debug, Default, Clone)]
pub struct NetworkLittleEndian;

impl Reader for BigEndian {
    fn u8(&mut self, buf: &mut impl Buf) -> Res<u8> {
        Ok(buf.get_u8())
    }

    fn i16(&mut self, buf: &mut impl Buf) -> Res<i16> {
        Ok(buf.get_i16())
    }

    fn i32(&mut self, buf: &mut impl Buf) -> Res<i32> {
        Ok(buf.get_i32())
    }

    fn i64(&mut self, buf: &mut impl Buf) -> Res<i64> {
        Ok(buf.get_i64())
    }

    fn f32(&mut self, buf: &mut impl Buf) -> Res<f32> {
        Ok(buf.get_f32())
    }

    fn f64(&mut self, buf: &mut impl Buf) -> Res<f64> {
        Ok(buf.get_f64())
    }
}

impl Writer for BigEndian {
    fn write_i16(&mut self, buf: &mut impl BufMut, x: i16) -> Res<()> {
        buf.put_i16(x);
        Ok(())
    }

    fn write_i32(&mut self, buf: &mut impl BufMut, x: i32) -> Res<()> {
        buf.put_i32(x);
        Ok(())
    }

    fn write_i64(&mut self, buf: &mut impl BufMut, x: i64) -> Res<()> {
        buf.put_i64(x);
        Ok(())
    }

    fn write_f32(&mut self, buf: &mut impl BufMut, x: f32) -> Res<()> {
        buf.put_f32(x);
        Ok(())
    }

    fn write_f64(&mut self, buf: &mut impl BufMut, x: f64) -> Res<()> {
        buf.put_f64(x);
        Ok(())
    }
}

impl Reader for LittleEndian {
    fn u8(&mut self, buf: &mut impl Buf) -> Res<u8> {
        Ok(buf.get_u8())
    }

    fn i16(&mut self, buf: &mut impl Buf) -> Res<i16> {
        Ok(buf.get_i16_le())
    }

    fn i32(&mut self, buf: &mut impl Buf) -> Res<i32> {
        Ok(buf.get_i32_le())
    }

    fn i64(&mut self, buf: &mut impl Buf) -> Res<i64> {
        Ok(buf.get_i64_le())
    }

    fn f32(&mut self, buf: &mut impl Buf) -> Res<f32> {
        Ok(buf.get_f32_le())
    }

    fn f64(&mut self, buf: &mut impl Buf) -> Res<f64> {
        Ok(buf.get_f64_le())
    }
}

impl Writer for LittleEndian {
    fn write_i16(&mut self, buf: &mut impl BufMut, x: i16) -> Res<()> {
        buf.put_i16_le(x);
        Ok(())
    }

    fn write_i32(&mut self, buf: &mut impl BufMut, x: i32) -> Res<()> {
        buf.put_i32_le(x);
        Ok(())
    }

    fn write_i64(&mut self, buf: &mut impl BufMut, x: i64) -> Res<()> {
        buf.put_i64_le(x);
        Ok(())
    }

    fn write_f32(&mut self, buf: &mut impl BufMut, x: f32) -> Res<()> {
        buf.put_f32_le(x);
        Ok(())
    }

    fn write_f64(&mut self, buf: &mut impl BufMut, x: f64) -> Res<()> {
        buf.put_f64_le(x);
        Ok(())
    }
}

impl Reader for NetworkLittleEndian {
    fn u8(&mut self, buf: &mut impl Buf) -> Res<u8> {
        Ok(buf.get_u8())
    }

    fn i16(&mut self, buf: &mut impl Buf) -> Res<i16> {
        Ok(buf.get_i16_le())
    }

    fn i32(&mut self, buf: &mut impl Buf) -> Res<i32> {
        let mut v: u32 = 0;
        for i in (0..35).step_by(7) {
            let b = self.u8(buf)?;

            v |= ((b & 0x7f) as u32) << i;
            if b & 0x80 == 0 {
                let x = (v >> 1) as i32;
                return Ok(if v & 1 != 0 { -x } else { x });
            }
        }
        Err(NbtError::ParseError("varint overflows integer".to_string()))
    }

    fn i64(&mut self, buf: &mut impl Buf) -> Res<i64> {
        let mut v: u64 = 0;
        for i in (0..70).step_by(7) {
            let b = self.u8(buf)?;

            v |= ((b & 0x7f) as u64) << i;
            if b & 0x80 == 0 {
                let x = (v >> 1) as i64;
                return Ok(if v & 1 != 0 { -x } else { x });
            }
        }
        Err(NbtError::ParseError("varint overflows integer".to_string()))
    }

    fn f32(&mut self, buf: &mut impl Buf) -> Res<f32> {
        Ok(buf.get_f32_le())
    }

    fn f64(&mut self, buf: &mut impl Buf) -> Res<f64> {
        Ok(buf.get_f64_le())
    }

    fn string(&mut self, buf: &mut impl Buf) -> Res<String> {
        let len = 'var_len: {
            let mut v: u32 = 0;
            for i in (0..35).step_by(7) {
                let b = self.u8(buf)?;

                v |= ((b & 0x7f) as u32) << i;
                if b & 0x80 == 0 {
                    break 'var_len v;
                }
            }
            return Err(NbtError::ParseError("varint overflows integer".to_string()));
        };

        let mut str_buf = Vec::with_capacity(len as usize);
        for _ in 0..len {
            str_buf.push(self.u8(buf)?);
        }

        Ok(String::from_utf8(str_buf)?)
    }
}

impl Writer for NetworkLittleEndian {
    fn write_u8(&mut self, buf: &mut impl BufMut, x: u8) -> Res<()> {
        buf.put_u8(x);
        Ok(())
    }

    fn write_i16(&mut self, buf: &mut impl BufMut, x: i16) -> Res<()> {
        buf.put_i16_le(x);
        Ok(())
    }

    fn write_i32(&mut self, buf: &mut impl BufMut, x: i32) -> Res<()> {
        let mut u = (x as u32) << 1;
        if x < 0 {
            u = !u;
        }
        while u >= 0x80 {
            self.write_u8(buf, u as u8 | 0x80)?;
            u >>= 7;
        }
        self.write_u8(buf, u as u8)?;
        Ok(())
    }

    fn write_i64(&mut self, buf: &mut impl BufMut, x: i64) -> Res<()> {
        let mut u = (x as u64) << 1;
        if x < 0 {
            u = !u;
        }
        while u >= 0x80 {
            self.write_u8(buf, u as u8 | 0x80)?;
            u >>= 7;
        }
        self.write_u8(buf, u as u8)?;
        Ok(())
    }

    fn write_f32(&mut self, buf: &mut impl BufMut, x: f32) -> Res<()> {
        buf.put_f32_le(x);
        Ok(())
    }

    fn write_f64(&mut self, buf: &mut impl BufMut, x: f64) -> Res<()> {
        buf.put_f64_le(x);
        Ok(())
    }

    fn write_string(&mut self, buf: &mut impl BufMut, x: &str) -> Res<()> {
        if x.len() > i16::MAX as usize {
            return Err(NbtError::ParseError("string too large".to_string()));
        }

        let mut l = x.len() as u32;
        while l >= 0x80 {
            self.write_u8(buf, l as u8 | 0x80)?;
            l >>= 7;
        }
        self.write_u8(buf, l as u8)?;
        for b in x.as_bytes() {
            self.write_u8(buf, *b)?;
        }
        Ok(())
    }
}

/// Test all encodings with various data.
#[cfg(test)]
mod tests {
    use crate::decode::Reader;
    use crate::encode::Writer;
    use crate::encoding::{BigEndian, LittleEndian, NetworkLittleEndian};
    use crate::NBTTag;
    use bytes::{Bytes, BytesMut};
    use std::collections::HashMap;

    #[test]
    fn test_big_endian() {
        test::<BigEndian>();
    }

    #[test]
    fn test_little_endian() {
        test::<LittleEndian>();
    }

    #[test]
    fn test_network_little_endian() {
        test::<NetworkLittleEndian>();
    }

    fn test<T: Reader + Writer + Sized + Default>() {
        let nbt = NBTTag::Compound(
            vec![
                ("test".to_string(), NBTTag::Long(10.into())),
                ("test1".to_string(), NBTTag::Byte(100.into())),
                ("test2".to_string(), NBTTag::Short(1.into())),
                (
                    "test3".to_string(),
                    NBTTag::List(
                        vec![
                            NBTTag::ByteArray(vec![1, 2, 3].into()),
                            NBTTag::ByteArray(vec![4, 5, 6].into()),
                        ]
                        .into(),
                    ),
                ),
                (
                    "test4".to_string(),
                    NBTTag::List(vec![NBTTag::Byte(1.into()), NBTTag::Byte(3.into())].into()),
                ),
                ("test5".to_string(), NBTTag::Compound(Default::default())),
            ]
            .iter()
            .cloned()
            .collect::<HashMap<String, NBTTag>>()
            .into(),
        );
        let mut buf_writer = BytesMut::default();
        nbt.write(&mut buf_writer, &mut T::default()).unwrap();

        let mut buf: Bytes = buf_writer.into();
        assert_eq!(NBTTag::read(&mut buf, &mut T::default()).unwrap(), nbt);
    }
}
