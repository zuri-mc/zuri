//! todo
use crate::Value;
use serde::{ser, Serialize};
use std::collections::HashMap;
use std::fmt::Display;
use serde::ser::{SerializeStruct, SerializeTuple};
use thiserror::Error;

/// Try to serialize a serde serializable type into NBT data.
pub fn serialize<T: Serialize>(input: T) -> Result<Value, SerializeError> {
    input.serialize(Serializer)
}

/// An error that can occur when serializing a struct into NBT data.
#[derive(Debug, Error)]
pub enum SerializeError {
    /// Error that occurs when two or more elements serialize into a different NBT type in a list
    /// type such as a [Vec].
    #[error("list entries must serialize to the same NBT type as first element in list")]
    MismatchedListType,
    /// Occurs when trying to serialize a map-like object that has a key that does not serialize to
    /// a [Value::String].
    #[error("key must be a string")]
    NonStringKey,
    /// Custom error that could be thrown by serde.
    #[error("{0}")]
    Custom(String),
}

struct Serializer;

impl ser::Error for SerializeError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::Custom(msg.to_string())
    }
}

fn wrap_enum(variant: &str, value: Value) -> Value {
    let mut map = HashMap::new();
    map.insert("variant".to_string(), Value::String(variant.to_string()));
    map.insert("value".to_string(), value);
    Value::Compound(map)
}

impl ser::Serializer for Serializer {
    type Ok = Value;
    type Error = SerializeError;

    type SerializeMap = CompoundSerializer;
    type SerializeStruct = CompoundSerializer;
    type SerializeSeq = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = CompoundSerializer;
    type SerializeTupleStruct = CompoundSerializer;
    type SerializeTupleVariant = CompoundVariantSerializer;
    type SerializeStructVariant = CompoundVariantSerializer;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Byte(v as u8))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Byte(v as u8))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Short(v))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Int(v))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Long(v))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Byte(v))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Short(v as i16))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Int(v as i32))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Long(v as i64))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Int(v as i32))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Float(v))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Double(v))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(Value::ByteArray(v.to_vec()))
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(CompoundSerializer::default())
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(CompoundSerializer::default())
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        Ok(Value::ByteArray((v as u128).to_le_bytes().to_vec()))
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        Ok(Value::ByteArray(v.to_le_bytes().to_vec()))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(Value::String(v.to_string()))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(wrap_enum("None", Value::Compound(Default::default())))
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Ok(wrap_enum("Some", value.serialize(Serializer)?))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Compound(Default::default()))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Compound(Default::default()))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(wrap_enum(variant, Value::Compound(Default::default())))
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(Serializer)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Ok(wrap_enum(variant, value.serialize(Serializer)?))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(CompoundSerializer::default())
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(CompoundSerializer::default())
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(CompoundVariantSerializer::new(variant))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(CompoundVariantSerializer::new(variant))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        unreachable!()
    }

    fn collect_seq<I>(self, iter: I) -> Result<Self::Ok, Self::Error>
    where
        I: IntoIterator,
        <I as IntoIterator>::Item: Serialize,
    {
        let mut iter = iter.into_iter();

        let first = iter.next();
        if first.is_none() {
            return Ok(Value::List(vec![]));
        }

        let first: Value = first.unwrap().serialize(Serializer)?;
        match first {
            Value::Byte(v) => {
                let mut list = vec![v];
                for item in iter {
                    list.push(
                        <Value as TryInto<u8>>::try_into(item.serialize(Serializer)?)
                            .map_err(|_| SerializeError::MismatchedListType)?,
                    )
                }

                Ok(Value::ByteArray(list))
            }
            Value::Int(v) => {
                let mut list = vec![v];
                for item in iter {
                    list.push(
                        <Value as TryInto<i32>>::try_into(item.serialize(Serializer)?)
                            .map_err(|_| SerializeError::MismatchedListType)?,
                    )
                }

                Ok(Value::IntArray(list))
            }
            Value::Long(v) => {
                let mut list = vec![v];
                for item in iter {
                    list.push(
                        <Value as TryInto<i64>>::try_into(item.serialize(Serializer)?)
                            .map_err(|_| SerializeError::MismatchedListType)?,
                    )
                }

                Ok(Value::LongArray(list))
            }
            v @ _ => {
                let tag_id = v.tag_id();

                let mut list = vec![v];
                for item in iter {
                    let new_value = item.serialize(Serializer)?;
                    if new_value.tag_id() != tag_id {
                        return Err(SerializeError::MismatchedListType);
                    }

                    list.push(new_value);
                }
                Ok(Value::List(list))
            }
        }
    }
}

/// Helper to serialize certain data types into a [Value::Compound].
#[derive(Default)]
struct CompoundSerializer {
    v: HashMap<String, Value>,
    index: usize,
}

impl ser::SerializeStruct for CompoundSerializer {
    type Ok = <Serializer as ser::Serializer>::Ok;
    type Error = <Serializer as ser::Serializer>::Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.v.insert(key.to_string(), value.serialize(Serializer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Compound(self.v))
    }
}

impl ser::SerializeMap for CompoundSerializer {
    type Ok = <Serializer as ser::Serializer>::Ok;
    type Error = <Serializer as ser::Serializer>::Error;

    fn serialize_key<T: ?Sized>(&mut self, _key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unreachable!()
    }

    fn serialize_value<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unreachable!()
    }

    fn serialize_entry<K: ?Sized, V: ?Sized>(
        &mut self,
        key: &K,
        value: &V,
    ) -> Result<(), Self::Error>
    where
        K: Serialize,
        V: Serialize,
    {
        let key = if let Value::String(str) = key.serialize(Serializer)? {
            str
        } else {
            return Err(SerializeError::NonStringKey);
        };
        self.v.insert(key, value.serialize(Serializer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Compound(self.v))
    }
}

impl ser::SerializeTuple for CompoundSerializer {
    type Ok = <Serializer as ser::Serializer>::Ok;
    type Error = <Serializer as ser::Serializer>::Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.v
            .insert(format!("{}", self.index), value.serialize(Serializer)?);
        self.index += 1;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Compound(self.v))
    }
}

impl ser::SerializeTupleStruct for CompoundSerializer {
    type Ok = <Serializer as ser::Serializer>::Ok;
    type Error = <Serializer as ser::Serializer>::Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where T: Serialize {
        self.serialize_element(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        <Self as ser::SerializeTuple>::end(self)
    }
}

/// Helper to serialize certain enum variants into a [Value::Compound].
struct CompoundVariantSerializer {
    inner: CompoundSerializer,
    variant: &'static str,
}

impl CompoundVariantSerializer {
    /// Create a new [CompoundVariantSerializer] for a certain variant.
    fn new(variant: &'static str) -> Self {
        Self {
            inner: Default::default(),
            variant,
        }
    }
}

impl ser::SerializeTupleVariant for CompoundVariantSerializer {
    type Ok = <Serializer as ser::Serializer>::Ok;
    type Error = <Serializer as ser::Serializer>::Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where T: Serialize {
        self.inner.serialize_element(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(wrap_enum(self.variant, <CompoundSerializer as SerializeTuple>::end(self.inner)?))
    }
}

impl ser::SerializeStructVariant for CompoundVariantSerializer {
    type Ok = <Serializer as ser::Serializer>::Ok;
    type Error = <Serializer as ser::Serializer>::Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> where T: Serialize {
        self.inner.serialize_field(key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(wrap_enum(self.variant, <CompoundSerializer as SerializeTuple>::end(self.inner)?))
    }
}

#[cfg(test)]
mod tests {
    use crate::serde::Serializer;
    use crate::Value;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[derive(Serialize, Deserialize)]
    pub struct ExampleStruct {
        test: i32,
        map: HashMap<String, bool>,
        vec: Vec<i16>,
        vec2: Vec<i32>,
        tuple: (String, u8, i64),
    }

    #[test]
    fn test_serialize() {
        // Construct
        let mut input = ExampleStruct {
            test: 7,
            map: HashMap::default(),
            vec: vec![1, 4, 6, 1],
            vec2: vec![1, 4, 6, 1],
            tuple: ("Test".to_string(), 1, 2),
        };
        input.map.insert("x".to_string(), true);
        input.map.insert("y".to_string(), false);
        input.map.insert("z".to_string(), false);

        let mut output_map = HashMap::new();
        output_map.insert("x".to_string(), Value::Byte(1));
        output_map.insert("y".to_string(), Value::Byte(0));
        output_map.insert("z".to_string(), Value::Byte(0));

        let mut output_tuple = HashMap::new();
        output_tuple.insert("0".to_string(), Value::String("Test".to_string()
        ));
        output_tuple.insert("1".to_string(), Value::Byte(1));
        output_tuple.insert("2".to_string(), Value::Long(2));

        let mut output = HashMap::new();
        output.insert("map".to_string(), Value::Compound(output_map));
        output.insert("tuple".to_string(), Value::Compound(output_tuple));
        output.insert("test".to_string(), Value::Int(7));
        output.insert(
            "vec".to_string(),
            Value::List(vec![
                Value::Short(1),
                Value::Short(4),
                Value::Short(6),
                Value::Short(1),
            ]),
        );
        output.insert("vec2".to_string(), Value::IntArray(vec![1, 4, 6, 1]));
        let output = Value::Compound(output);

        assert_eq!(input.serialize(Serializer).unwrap(), output)
    }
}
