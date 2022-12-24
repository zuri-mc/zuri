use std::collections::HashMap;
use std::fmt::Debug;

use encode::Writer;

use crate::decode::Reader;
use crate::err::{NbtError, Res};

mod encode;
mod decode;
mod err;

#[derive(Debug, Clone)]
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

    pub fn read(r: &mut impl Reader) -> Res<Self> {
        let tag_id = r.u8()?;
        r.string()?;
        Self::read_inner(tag_id, r)
    }

    fn read_inner(tag_id: u8, r: &mut impl Reader) -> Res<Self> {
        Ok(match tag_id {
            1 => Value::Byte(r.u8()?),
            2 => Value::Short(r.i16()?),
            3 => Value::Int(r.i32()?),
            4 => Value::Long(r.i64()?),
            5 => Value::Float(r.f32()?),
            6 => Value::Double(r.f64()?),
            8 => Value::String(r.string()?),
            10 => {
                let mut map = HashMap::new();
                loop {
                    let content_type = r.u8()?;
                    if content_type == 0 {
                        break;
                    }
                    let name = r.string()?;
                    map.insert(name, Self::read_inner(content_type, r)?);
                }
                Value::Compound(map)
            }
            9 => {
                let content_type = r.u8()?;
                let len = r.i32()?;
                if len <= 0 {
                    return Err(NbtError::ParseError("list length must be greater than 0".to_string()));
                }
                let mut vec = Vec::with_capacity(len as usize);
                for _ in 0..len {
                    vec.push(Self::read_inner(content_type, r)?);
                }
                Value::List(vec)
            }
            7 => Value::ByteArray(r.u8_vec()?),
            11 => Value::IntArray(r.i32_vec()?),
            12 => Value::LongArray(r.i64_vec()?),
            _ => panic!("Unknown tag type {}", tag_id),
        })
    }

    pub fn write(&self, w: &mut impl Writer) -> Res<()> {
        w.u8(self.tag_id())?;
        w.string("")?;
        self.write_inner(w)
    }

    fn write_inner(&self, w: &mut impl Writer) -> Res<()> {
        match self {
            Self::Byte(x) => w.u8(*x)?,
            Self::Short(x) => w.i16(*x)?,
            Self::Int(x) => w.i32(*x)?,
            Self::Long(x) => w.i64(*x)?,
            Self::Float(x) => w.f32(*x)?,
            Self::Double(x) => w.f64(*x)?,
            Self::String(x) => w.string(x.as_str())?,
            Self::Compound(x) => {
                for (name, val) in x {
                    w.u8(val.tag_id())?;
                    w.string(name)?;
                    val.write_inner(w)?;
                }
                w.end()?;
            }
            Self::List(x) => {
                if x.is_empty() {
                    w.u8_vec(&Vec::<u8>::new())?;
                    return Ok(());
                }
                let first_id = x[0].tag_id();

                w.u8(first_id)?;
                w.i32(x.len() as i32)?;
                for v in x {
                    if v.tag_id() != first_id {
                        return Err(NbtError::ParseError("list elements must be of same type".to_string()));
                    }
                    v.write_inner(w)?;
                }
            }
            Self::ByteArray(x) => {
                w.u8_vec(x)?
            }
            Self::IntArray(x) => w.i32_vec(x)?,
            Self::LongArray(x) => w.i64_vec(x)?,
        };
        Ok(())
    }
}
