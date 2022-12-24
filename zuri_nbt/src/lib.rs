use std::collections::HashMap;
use std::fmt::Debug;

use bytes::{Buf, BufMut};

use encode::Writer;

use crate::decode::Reader;
use crate::err::{NbtError, Res};

mod encode;
mod decode;
mod err;
mod encoding;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Byte(u8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    String(String),
    Compound(HashMap<String, Value>),
    List(Vec<Value>),
    ByteArray(Vec<u8>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

impl Value {
    fn tag_id(&self) -> u8 {
        match self {
            Value::Byte(_) => 1,
            Value::Short(_) => 2,
            Value::Int(_) => 3,
            Value::Long(_) => 4,
            Value::Float(_) => 5,
            Value::Double(_) => 6,
            Value::String(_) => 8,
            Value::Compound(_) => 10,
            Value::List(_) => 9,
            Value::ByteArray(_) => 7,
            Value::IntArray(_) => 11,
            Value::LongArray(_) => 12,
        }
    }

    pub fn read(buf: &mut impl Buf, r: &mut impl Reader) -> Res<Self> {
        let tag_id = r.u8(buf)?;
        r.string(buf)?;
        Self::read_inner(buf, tag_id, r)
    }

    pub fn write(&self, buf: &mut impl BufMut, w: &mut impl Writer) -> Res<()> {
        w.write_u8(buf, self.tag_id())?;
        w.write_string(buf, "")?;
        self.write_inner(buf, w)
    }

    fn read_inner(buf: &mut impl Buf, tag_id: u8, r: &mut impl Reader) -> Res<Self> {
        Ok(match tag_id {
            1 => Value::Byte(r.u8(buf)?),
            2 => Value::Short(r.i16(buf)?),
            3 => Value::Int(r.i32(buf)?),
            4 => Value::Long(r.i64(buf)?),
            5 => Value::Float(r.f32(buf)?),
            6 => Value::Double(r.f64(buf)?),
            8 => Value::String(r.string(buf)?),
            10 => {
                let mut map = HashMap::new();
                loop {
                    let content_type = r.u8(buf)?;
                    if content_type == 0 {
                        break;
                    }
                    let name = r.string(buf)?;
                    map.insert(name, Self::read_inner(buf, content_type, r)?);
                }
                Value::Compound(map)
            }
            9 => {
                let content_type = r.u8(buf)?;
                let len = r.i32(buf)?;
                if len <= 0 {
                    return Err(NbtError::ParseError("list length must be greater than 0".to_string()));
                }
                let mut vec = Vec::with_capacity(len as usize);
                for _ in 0..len {
                    vec.push(Self::read_inner(buf, content_type, r)?);
                }
                Value::List(vec)
            }
            7 => Value::ByteArray(r.u8_vec(buf)?),
            11 => Value::IntArray(r.i32_vec(buf)?),
            12 => Value::LongArray(r.i64_vec(buf)?),
            _ => panic!("Unknown tag type {}", tag_id),
        })
    }

    fn write_inner(&self, buf: &mut impl BufMut, w: &mut impl Writer) -> Res<()> {
        match self {
            Self::Byte(x) => w.write_u8(buf, *x)?,
            Self::Short(x) => w.write_i16(buf, *x)?,
            Self::Int(x) => w.write_i32(buf, *x)?,
            Self::Long(x) => w.write_i64(buf, *x)?,
            Self::Float(x) => w.write_f32(buf, *x)?,
            Self::Double(x) => w.write_f64(buf, *x)?,
            Self::String(x) => w.write_string(buf, x.as_str())?,
            Self::Compound(x) => {
                for (name, val) in x {
                    w.write_u8(buf, val.tag_id())?;
                    w.write_string(buf, name)?;
                    val.write_inner(buf, w)?;
                }
                w.write_end(buf)?;
            }
            Self::List(x) => {
                if x.is_empty() {
                    w.write_u8_vec(buf, &Vec::<u8>::new())?;
                    return Ok(());
                }
                let first_id = x[0].tag_id();

                w.write_u8(buf, first_id)?;
                w.write_i32(buf, x.len() as i32)?;
                for v in x {
                    if v.tag_id() != first_id {
                        return Err(NbtError::ParseError("list elements must be of same type".to_string()));
                    }
                    v.write_inner(buf, w)?;
                }
            }
            Self::ByteArray(x) => {
                w.write_u8_vec(buf, x)?
            }
            Self::IntArray(x) => w.write_i32_vec(buf, x)?,
            Self::LongArray(x) => w.write_i64_vec(buf, x)?,
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use bytes::{Bytes, BytesMut};
    use crate::decode::Reader;
    use crate::encode::Writer;
    use crate::encoding::{LittleEndian, NetworkLittleEndian};
    use crate::Value;

    #[test]
    fn test_little_endian() {
        test::<LittleEndian>();
    }

    #[test]
    fn test_network_little_endian() {
        test::<NetworkLittleEndian>();
    }

    fn test<T: Reader + Writer + Sized + Default>() {
        let nbt = Value::Compound(vec![("test".to_string(), Value::Long(10)), ("test".to_string(), Value::List(vec![Value::Byte(1), Value::Byte(3)]))].iter().cloned().collect());
        let mut buf_writer = BytesMut::default();
        nbt.write(&mut buf_writer, &mut T::default()).unwrap();

        let mut buf: Bytes = buf_writer.into();
        assert_eq!(Value::read(&mut buf, &mut T::default()).unwrap(), nbt);
    }
}
