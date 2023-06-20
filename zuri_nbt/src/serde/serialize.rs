use crate::err::{ErrorPath, PathPart};
use crate::serde::SerializeError;
use crate::{err, tag, NBTTag};
use serde::{ser, Serialize};
use std::collections::HashMap;

pub(super) struct Serializer;

fn wrap_enum(variant: &str, value: NBTTag) -> NBTTag {
    let mut map = HashMap::new();
    map.insert(
        "variant".to_string(),
        NBTTag::String(variant.to_string().into()),
    );
    map.insert("value".to_string(), value);
    NBTTag::Compound(map.into())
}

impl ser::Serializer for Serializer {
    type Ok = NBTTag;
    type Error = ErrorPath<SerializeError>;

    type SerializeMap = CompoundSerializer;
    type SerializeStruct = CompoundSerializer;
    type SerializeSeq = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = CompoundSerializer;
    type SerializeTupleStruct = CompoundSerializer;
    type SerializeTupleVariant = CompoundVariantSerializer;
    type SerializeStructVariant = CompoundVariantSerializer;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(NBTTag::Byte((v as u8).into()))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(NBTTag::Byte((v as u8).into()))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(NBTTag::Short(v.into()))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(NBTTag::Int(v.into()))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(NBTTag::Long(v.into()))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(NBTTag::Byte(v.into()))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(NBTTag::Short((v as i16).into()))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(NBTTag::Int((v as i32).into()))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(NBTTag::Long((v as i64).into()))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(NBTTag::Int((v as i32).into()))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(NBTTag::Float(v.into()))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(NBTTag::Double(v.into()))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(NBTTag::ByteArray(v.to_vec().into()))
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
        Ok(NBTTag::ByteArray((v as u128).to_le_bytes().to_vec().into()))
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        Ok(NBTTag::ByteArray(v.to_le_bytes().to_vec().into()))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(NBTTag::String(v.to_string().into()))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(wrap_enum("None", NBTTag::Compound(Default::default())))
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Ok(wrap_enum("Some", value.serialize(Serializer)?))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(NBTTag::Compound(Default::default()))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(NBTTag::Compound(Default::default()))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(wrap_enum(variant, NBTTag::Compound(Default::default())))
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
            return Ok(NBTTag::List(Default::default()));
        }

        let first: NBTTag = first.unwrap().serialize(Serializer)?;
        match first {
            NBTTag::Byte(v) => {
                let mut list = vec![v.0];
                for (i, item) in iter.enumerate() {
                    list.push(
                        <NBTTag as TryInto<tag::Byte>>::try_into(item.serialize(Serializer)?)
                            .map_err(|_| {
                                ErrorPath::new_with_path(
                                    SerializeError::MismatchedListType,
                                    err::Path::from_single(err::PathPart::Element(i + 1)),
                                )
                            })?
                            .0,
                    )
                }

                Ok(NBTTag::ByteArray(list.into()))
            }
            NBTTag::Int(v) => {
                let mut list = vec![v.0];
                for (i, item) in iter.enumerate() {
                    list.push(
                        <NBTTag as TryInto<tag::Int>>::try_into(item.serialize(Serializer)?)
                            .map_err(|_| {
                                ErrorPath::new_with_path(
                                    SerializeError::MismatchedListType,
                                    err::Path::from_single(err::PathPart::Element(i + 1)),
                                )
                            })?
                            .0,
                    )
                }

                Ok(NBTTag::IntArray(list.into()))
            }
            NBTTag::Long(v) => {
                let mut list = vec![v.0];
                for (i, item) in iter.enumerate() {
                    list.push(
                        <NBTTag as TryInto<tag::Long>>::try_into(item.serialize(Serializer)?)
                            .map_err(|_| {
                                ErrorPath::new_with_path(
                                    SerializeError::MismatchedListType,
                                    err::Path::from_single(err::PathPart::Element(i + 1)),
                                )
                            })?
                            .0,
                    )
                }

                Ok(NBTTag::LongArray(list.into()))
            }
            v => {
                let tag_id = v.tag_id();

                let mut list = vec![v];
                for (i, item) in iter.enumerate() {
                    let new_value = item.serialize(Serializer)?;
                    if new_value.tag_id() != tag_id {
                        return Err(ErrorPath::new_with_path(
                            SerializeError::MismatchedListType,
                            err::Path::from_single(err::PathPart::Element(i + 1)),
                        ));
                    }

                    list.push(new_value);
                }
                Ok(NBTTag::List(list.into()))
            }
        }
    }
}

/// Helper to serialize certain data types into a [NBTTag::Compound].
#[derive(Default)]
pub(super) struct CompoundSerializer {
    v: HashMap<String, NBTTag>,
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
        self.v.insert(
            key.to_string(),
            value
                .serialize(Serializer)
                .map_err(|err| err.prepend(PathPart::Field(key.to_string())))?,
        );
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(NBTTag::Compound(self.v.into()))
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
        let key_str = if let NBTTag::String(str) = key.serialize(Serializer)? {
            str
        } else {
            return Err(ErrorPath::new(SerializeError::NonStringKey));
        };
        self.v.insert(
            key_str.0,
            value.serialize(Serializer).map_err(|err| {
                err.prepend(PathPart::MapKey(
                    // The key has moved into the map, so we need to serialize it again.
                    <NBTTag as TryInto<tag::String>>::try_into(key.serialize(Serializer).unwrap())
                        .unwrap()
                        .0,
                ))
            })?,
        );
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(NBTTag::Compound(self.v.into()))
    }
}

impl ser::SerializeTuple for CompoundSerializer {
    type Ok = <Serializer as ser::Serializer>::Ok;
    type Error = <Serializer as ser::Serializer>::Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.v.insert(
            format!("{}", self.index),
            value
                .serialize(Serializer)
                .map_err(|err| err.prepend(PathPart::TupleField(self.index)))?,
        );
        self.index += 1;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(NBTTag::Compound(self.v.into()))
    }
}

impl ser::SerializeTupleStruct for CompoundSerializer {
    type Ok = <Serializer as ser::Serializer>::Ok;
    type Error = <Serializer as ser::Serializer>::Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        <Self as ser::SerializeTuple>::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        <Self as ser::SerializeTuple>::end(self)
    }
}

/// Helper to serialize certain enum variants into a [NBTTag::Compound].
pub(super) struct CompoundVariantSerializer {
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

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        <CompoundSerializer as ser::SerializeTuple>::serialize_element(&mut self.inner, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(wrap_enum(
            self.variant,
            <CompoundSerializer as ser::SerializeTuple>::end(self.inner)?,
        ))
    }
}

impl ser::SerializeStructVariant for CompoundVariantSerializer {
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
        <CompoundSerializer as ser::SerializeStruct>::serialize_field(&mut self.inner, key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(wrap_enum(
            self.variant,
            <CompoundSerializer as ser::SerializeTuple>::end(self.inner)?,
        ))
    }
}
