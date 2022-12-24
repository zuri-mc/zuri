use std::any::Any;
use std::collections::HashMap;

use encode::Writer;
use crate::decode::Reader;

mod encode;
mod decode;

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

    pub fn read(r: &mut impl Reader) -> Self {
        let tag_id = r.read_u8();
        r.read_string();
        Self::read_inner(tag_id, r)
    }

    fn read_inner(tag_id: u8, r: &mut impl Reader) -> Self {
        match tag_id {
            1 => Value::Byte(r.read_u8()),
            2 => Value::Short(r.read_i16()),
            3 => Value::Int(r.read_i32()),
            4 => Value::Long(r.read_i64()),
            5 => Value::Float(r.read_f32()),
            6 => Value::Double(r.read_f64()),
            8 => Value::String(r.read_string()),
            10 => {
                let mut map = HashMap::new();
                loop {
                    let content_type = r.read_u8();
                    if content_type == 0 {
                        break;
                    }
                    let name = r.read_string();
                    map.insert(name, Self::read_inner(content_type, r));
                }
                Value::Compound(map)
            }
            9 => {
                let content_type = r.read_u8();
                let len = r.read_i32();
                if len <= 0 {
                    panic!()
                }
                let mut vec = Vec::with_capacity(len as usize);
                for _ in 0..len {
                    vec.push(Self::read_inner(content_type, r));
                }
                Value::List(vec)
            }
            7 => Value::ByteArray(r.read_u8_vec()),
            11 => Value::IntArray(r.read_i32_vec()),
            12 => Value::LongArray(r.read_i64_vec()),
            _ => panic!("Unknown tag type {}", tag_id),
        }
    }

    pub fn write(&self, w: &mut impl Writer) {
        w.write_u8(self.tag_id());
        w.write_string("");
        self.write_inner(w);
    }

    fn write_inner(&self, w: &mut impl Writer) {
        match self {
            Self::Byte(x) => w.write_u8(*x),
            Self::Short(x) => w.write_i16(*x),
            Self::Int(x) => w.write_i32(*x),
            Self::Long(x) => w.write_i64(*x),
            Self::Float(x) => w.write_f32(*x),
            Self::Double(x) => w.write_f64(*x),
            Self::String(x) => w.write_string(x.as_str()),
            Self::Compound(x) => {
                for (name, val) in x {
                    w.write_u8(val.tag_id());
                    w.write_string(name);
                    val.write_inner(w);
                }
                w.write_end();
            }
            Self::List(x) => {
                if x.is_empty() {
                    w.write_u8_vec(&Vec::<u8>::new());
                    return;
                }
                let first_id = x[0].tag_id();

                w.write_u8(first_id);
                w.write_i32(x.len() as i32);
                for v in x {
                    if v.tag_id() != first_id {
                        panic!("Wrong type"); // todo: Result
                    }
                    v.write_inner(w);
                }
            }
            Self::ByteArray(x) => {
                w.write_u8_vec(x)
            },
            Self::IntArray(x) => w.write_i32_vec(x),
            Self::LongArray(x) => w.write_i64_vec(x),
        };
    }
}
