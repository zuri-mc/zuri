use crate::err::PathPart;
use crate::serde::{DeserializeError, ErrorPath};
use crate::NBTTag;
use serde::de;
use serde::de::{DeserializeSeed, Visitor};
use std::collections::{hash_map, HashMap};

pub(super) struct Deserializer<'de> {
    nbt: &'de NBTTag,
}

impl<'de> Deserializer<'de> {
    pub fn new(input: &'de NBTTag) -> Self {
        Self { nbt: input }
    }
}

impl<'de> de::Deserializer<'de> for Deserializer<'de> {
    type Error = ErrorPath<DeserializeError>;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match &self.nbt {
            NBTTag::Byte(_) => self.deserialize_u8(visitor),
            NBTTag::Short(_) => self.deserialize_i16(visitor),
            NBTTag::Int(_) => self.deserialize_i32(visitor),
            NBTTag::Long(_) => self.deserialize_i64(visitor),
            NBTTag::Float(_) => self.deserialize_f32(visitor),
            NBTTag::Double(_) => self.deserialize_f64(visitor),
            NBTTag::String(_) => self.deserialize_string(visitor),
            NBTTag::Compound(_) => self.deserialize_map(visitor),
            NBTTag::List(_) => self.deserialize_seq(visitor),
            NBTTag::ByteArray(_) => self.deserialize_seq(visitor),
            NBTTag::IntArray(_) => self.deserialize_seq(visitor),
            NBTTag::LongArray(_) => self.deserialize_seq(visitor),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.nbt {
            NBTTag::Byte(v) => visitor.visit_bool(v.0 != 0),
            _ => Err(ErrorPath::new(DeserializeError::UnexpectedTag)),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.nbt {
            NBTTag::Byte(v) => visitor.visit_i8(v.0 as i8),
            _ => Err(ErrorPath::new(DeserializeError::UnexpectedTag)),
        }
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.nbt {
            NBTTag::Short(v) => visitor.visit_i16(v.0),
            _ => Err(ErrorPath::new(DeserializeError::UnexpectedTag)),
        }
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.nbt {
            NBTTag::Int(v) => visitor.visit_i32(v.0),
            _ => Err(ErrorPath::new(DeserializeError::UnexpectedTag)),
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.nbt {
            NBTTag::Long(v) => visitor.visit_i64(v.0),
            _ => Err(ErrorPath::new(DeserializeError::UnexpectedTag)),
        }
    }

    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let NBTTag::ByteArray(v) = self.nbt {
            visitor.visit_i128(u128::from_le_bytes(
                v.0[0..std::mem::size_of::<i128>()]
                    .try_into()
                    .map_err(|_| ErrorPath::new(DeserializeError::InvalidConversion))?,
            ) as i128)
        } else {
            Err(ErrorPath::new(DeserializeError::UnexpectedTag))
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.nbt {
            NBTTag::Byte(v) => visitor.visit_u8(v.0),
            _ => Err(ErrorPath::new(DeserializeError::UnexpectedTag)),
        }
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.nbt {
            NBTTag::Short(v) => visitor.visit_u16(v.0 as u16),
            _ => Err(ErrorPath::new(DeserializeError::UnexpectedTag)),
        }
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.nbt {
            NBTTag::Int(v) => visitor.visit_u32(v.0 as u32),
            _ => Err(ErrorPath::new(DeserializeError::UnexpectedTag)),
        }
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.nbt {
            NBTTag::Long(v) => visitor.visit_u64(v.0 as u64),
            _ => Err(ErrorPath::new(DeserializeError::UnexpectedTag)),
        }
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let NBTTag::ByteArray(v) = self.nbt {
            visitor.visit_u128(u128::from_le_bytes(
                v.0[0..std::mem::size_of::<u128>()]
                    .try_into()
                    .map_err(|_| ErrorPath::new(DeserializeError::InvalidConversion))?,
            ))
        } else {
            Err(ErrorPath::new(DeserializeError::UnexpectedTag))
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.nbt {
            NBTTag::Float(v) => visitor.visit_f32(v.0),
            _ => Err(ErrorPath::new(DeserializeError::UnexpectedTag)),
        }
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.nbt {
            NBTTag::Double(v) => visitor.visit_f64(v.0),
            _ => Err(ErrorPath::new(DeserializeError::UnexpectedTag)),
        }
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.nbt {
            NBTTag::Int(v) => visitor.visit_char(
                char::from_u32(v.0 as u32)
                    .ok_or(ErrorPath::new(DeserializeError::InvalidConversion))?,
            ),
            _ => Err(ErrorPath::new(DeserializeError::UnexpectedTag)),
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.nbt {
            NBTTag::String(v) => visitor.visit_borrowed_str(v.0.as_str()),
            _ => Err(ErrorPath::new(DeserializeError::UnexpectedTag)),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.nbt {
            NBTTag::String(v) => visitor.visit_borrowed_str(v.0.as_str()),
            _ => Err(ErrorPath::new(DeserializeError::UnexpectedTag)),
        }
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.nbt {
            NBTTag::ByteArray(v) => visitor.visit_borrowed_bytes(v.0.as_slice()),
            _ => Err(ErrorPath::new(DeserializeError::UnexpectedTag)),
        }
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.nbt {
            NBTTag::ByteArray(v) => visitor.visit_byte_buf(v.0.clone()),
            _ => Err(ErrorPath::new(DeserializeError::UnexpectedTag)),
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let NBTTag::Compound(map) = self.nbt {
            let variant = map
                .0
                .get("variant")
                .ok_or(ErrorPath::new(DeserializeError::UnexpectedVariant))?;
            let variant = if let NBTTag::String(v) = variant {
                v.0.as_str()
            } else {
                return Err(ErrorPath::new(DeserializeError::UnexpectedVariant));
            };
            match variant {
                "None" => visitor.visit_none(),
                "Some" => {
                    let value = map
                        .0
                        .get("value")
                        .ok_or(ErrorPath::new(DeserializeError::UnexpectedVariant))?;
                    visitor.visit_some(Deserializer::new(value))
                }
                _ => Err(ErrorPath::new(DeserializeError::UnexpectedVariant)),
            }
        } else {
            Err(ErrorPath::new(DeserializeError::UnexpectedTag))
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let NBTTag::Compound(_) = self.nbt {
            visitor.visit_unit()
        } else {
            Err(ErrorPath::new(DeserializeError::UnexpectedTag))
        }
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.nbt {
            NBTTag::List(v) => visitor.visit_seq(ListAccess {
                iter: v.0.iter(),
                elems: 0,
            }),
            NBTTag::ByteArray(v) => {
                visitor.visit_seq(de::value::SeqDeserializer::new(v.0.iter().cloned()))
            }
            NBTTag::IntArray(v) => {
                visitor.visit_seq(de::value::SeqDeserializer::new(v.0.iter().cloned()))
            }
            NBTTag::LongArray(v) => {
                visitor.visit_seq(de::value::SeqDeserializer::new(v.0.iter().cloned()))
            }
            _ => Err(ErrorPath::new(DeserializeError::UnexpectedTag)),
        }
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let NBTTag::Compound(map) = self.nbt {
            visitor.visit_seq(TupleAccess {
                map: &map.0,
                next: 0,
            })
        } else {
            Err(ErrorPath::new(DeserializeError::UnexpectedTag))
        }
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let NBTTag::Compound(map) = self.nbt {
            visitor.visit_map(CompoundAccess {
                map_iter: map.0.iter(),
                next_value: None,
            })
        } else {
            Err(ErrorPath::new(DeserializeError::UnexpectedTag))
        }
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let NBTTag::Compound(map) = self.nbt {
            visitor.visit_enum(EnumAccess { map: &map.0 })
        } else {
            Err(ErrorPath::new(DeserializeError::UnexpectedTag))
        }
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.nbt {
            NBTTag::String(v) => visitor.visit_str(v.0.as_str()),
            _ => Err(ErrorPath::new(DeserializeError::UnexpectedTag)),
        }
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

impl<'de> de::VariantAccess<'de> for Deserializer<'de> {
    type Error = ErrorPath<DeserializeError>;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        seed.deserialize(self)
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        <Self as de::Deserializer>::deserialize_tuple(self, len, visitor)
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        <Self as de::Deserializer>::deserialize_map(self, visitor)
    }
}

struct EnumAccess<'de> {
    map: &'de HashMap<String, NBTTag>,
}

impl<'de> de::EnumAccess<'de> for EnumAccess<'de> {
    type Error = ErrorPath<DeserializeError>;
    type Variant = Deserializer<'de>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        if let Some(variant) = self.map.get("variant") {
            if let Some(value) = self.map.get("value") {
                Ok((
                    seed.deserialize(Deserializer::new(variant))?,
                    Deserializer::new(value),
                ))
            } else {
                Err(ErrorPath::new(DeserializeError::UnexpectedVariant))
            }
        } else {
            Err(ErrorPath::new(DeserializeError::UnexpectedVariant))
        }
    }
}

struct ListAccess<'de, I: Iterator<Item = &'de NBTTag>> {
    iter: I,
    elems: usize,
}

impl<'de, I: Iterator<Item = &'de NBTTag>> de::SeqAccess<'de> for ListAccess<'de, I> {
    type Error = ErrorPath<DeserializeError>;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        if let Some(next) = self.iter.next() {
            let elem = self.elems;
            self.elems += 1;
            Ok(Some(seed.deserialize(Deserializer::new(next)).map_err(
                |mut err| {
                    err.path.0.push_front(PathPart::Element(elem));
                    err
                },
            )?))
        } else {
            Ok(None)
        }
    }
}

struct TupleAccess<'de> {
    map: &'de HashMap<String, NBTTag>,
    next: usize,
}

impl<'de> de::SeqAccess<'de> for TupleAccess<'de> {
    type Error = ErrorPath<DeserializeError>;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        if let Some(next) = self.map.get_key_value(&self.next.to_string()) {
            let num = self.next;
            self.next += 1;

            Ok(Some(seed.deserialize(Deserializer::new(next.1)).map_err(
                |mut err| {
                    err.path.0.push_front(PathPart::TupleField(num));
                    err
                },
            )?))
        } else {
            Ok(None)
        }
    }
}

struct CompoundAccess<'de> {
    map_iter: hash_map::Iter<'de, String, NBTTag>,
    next_value: Option<(&'de str, &'de NBTTag)>,
}

impl<'de> de::MapAccess<'de> for CompoundAccess<'de> {
    type Error = ErrorPath<DeserializeError>;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        if let Some((k, v)) = self.map_iter.next() {
            self.next_value = Some((k.as_str(), v));
            return Ok(Some(
                seed.deserialize(de::value::StrDeserializer::<'de, Self::Error>::new(
                    k.as_str(),
                ))
                .map_err(|mut err| {
                    err.path.0.push_front(PathPart::Field(k.to_string()));
                    err
                })?,
            ));
        }
        Ok(None)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        if self.next_value.is_none() {
            panic!("calling next_value_seed before next_key_seed");
        }

        let (k, v) = self.next_value.unwrap();

        let res = Ok(seed.deserialize(Deserializer::new(v)).map_err(|mut err| {
            err.path.0.push_front(PathPart::Field(k.to_string()));
            err
        })?);
        self.next_value = None;
        res
    }

    fn next_entry_seed<K, V>(
        &mut self,
        kseed: K,
        vseed: V,
    ) -> Result<Option<(K::Value, V::Value)>, Self::Error>
    where
        K: DeserializeSeed<'de>,
        V: DeserializeSeed<'de>,
    {
        if let Some((k, v)) = self.map_iter.next() {
            return Ok(Some((
                kseed
                    .deserialize(de::value::StrDeserializer::<'de, Self::Error>::new(
                        k.as_str(),
                    ))
                    .map_err(|mut err| {
                        err.path.0.push_front(PathPart::Field(k.to_string()));
                        err
                    })?,
                vseed.deserialize(Deserializer::new(v)).map_err(|mut err| {
                    err.path.0.push_front(PathPart::Field(k.to_string()));
                    err
                })?,
            )));
        }
        Ok(None)
    }
}
