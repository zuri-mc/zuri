//! Enables the serialization and deserialization of rust types into NBT data through the
//! [serialize] and [deserialize] functions respectively.
//!
//! ## Example
//! To be able to deserialize and serialize your types, you will need to derive
//! [Deserialize](Deserialize) and [Serialize](Serialize) respectively.
//! ```no_run
//! # use serde::{Deserialize, Serialize};
//! # use zuri_nbt::serde::{deserialize, serialize};
//! # use zuri_nbt::NBTTag;
//! # #[derive(Default)]
//! #[derive(Deserialize, Serialize)]
//! struct MyStruct {
//!     some_field: String,
//!     other_field: Option<u8>,
//! }
//!
//! // Deserializing NBT into MyStruct
//! # let nbt = NBTTag::default();
//! let my_struct: MyStruct = deserialize(&nbt).expect("Could not deserialize");
//!
//! // Serializing MyStruct into NBT
//! # let my_struct = MyStruct::default();
//! let nbt = serialize(&my_struct).expect("Could not serialize");
//! ```
mod deserialize;
mod serialize;

use crate::err::ErrorPath;
use crate::serde::deserialize::Deserializer;
use crate::serde::serialize::Serializer;
use crate::NBTTag;
use serde::{de, ser, Deserialize, Serialize};
use std::fmt::Display;
use thiserror::Error;

/// Try to serialize a serde serializable type into NBT data.
pub fn serialize<T: Serialize>(input: &T) -> Result<NBTTag, SerializeError> {
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
    /// a [NBTTag::String].
    #[error("key must be a string")]
    NonStringKey,
    /// Custom error that could be thrown by serde.
    #[error("{0}")]
    Custom(String),
}

/// Deserialize NBT data into a data type.
pub fn deserialize<'de, T: Deserialize<'de>>(
    input: &'de NBTTag,
) -> Result<T, ErrorPath<'de, DeserializeError>> {
    T::deserialize(Deserializer::<'de>::new(input))
}

/// An error that can occur when deserializing data.
///
/// Generally, this indicates that the input data is invalid.
#[derive(Debug, Error)]
pub enum DeserializeError {
    /// An enum was expected but the compound tag does not have a known enum variant or is missing
    /// the `variant` or `value` tag.
    #[error("unexpected or missing enum variant or value")]
    UnexpectedVariant,
    /// A type conversion has resulted in an error.
    #[error("could not convert value")]
    InvalidConversion,
    /// A tag in the input was not of the expected type.
    #[error("unexpected tag type in input")]
    UnexpectedTag,
    /// Occurs when trying to deserialize into a map-like object that has a non-stringlike key.
    #[error("key must be a string")]
    NonStringKey,
    /// Custom error that could be thrown by serde.
    #[error("{0}")]
    Custom(String),
}

impl<'a, I: ser::Error + 'static> ser::Error for ErrorPath<'a, I> {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self {
            inner: I::custom(msg),
            path: Default::default(),
        }
    }
}

impl<'a, I: de::Error + 'static> de::Error for ErrorPath<'a, I> {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self {
            inner: I::custom(msg),
            path: Default::default(),
        }
    }
}

impl de::Error for DeserializeError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::Custom(msg.to_string())
    }
}

impl ser::Error for SerializeError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::Custom(msg.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::serde::{deserialize, serialize};
    use crate::{tag, NBTTag};
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    pub struct ExampleStruct<'a> {
        test: i32,
        map: HashMap<String, bool>,
        vec0: Vec<u8>,
        vec1: Vec<i16>,
        vec2: Vec<i32>,
        tuple: (String, u8, i64),
        option0: Option<&'a str>,
        option1: Option<&'a str>,
        enum0: ExampleEnum,
        enum1: ExampleEnum,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    pub enum ExampleEnum {
        Unit,
        Tuple(i8, u8),
    }

    #[test]
    fn test_serialize_deserialize() {
        let mut input = ExampleStruct {
            test: 7,
            map: HashMap::default(),
            vec0: vec![1, 4, 6, 1],
            vec1: vec![1, 4, 6, 1],
            vec2: vec![1, 4, 6, 1],
            tuple: ("Test".to_string(), 1, 2),
            option0: Some("hi"),
            option1: None,
            enum0: ExampleEnum::Unit,
            enum1: ExampleEnum::Tuple(1, 2),
        };
        input.map.insert("x".to_string(), true);
        input.map.insert("y".to_string(), false);
        input.map.insert("z".to_string(), false);

        let output = tag::Compound::builder()
            .with::<tag::Compound>(
                "enum0",
                tag::Compound::builder()
                    .with::<tag::String>("variant", "Unit".to_string().into())
                    .with("value", tag::Compound::default())
                    .build(),
            )
            .with::<tag::Compound>(
                "enum1",
                tag::Compound::builder()
                    .with::<tag::String>("variant", "Tuple".to_string().into())
                    .with::<tag::Compound>(
                        "value",
                        tag::Compound::builder()
                            .with::<tag::Byte>("0", 1.into())
                            .with::<tag::Byte>("1", 2.into())
                            .build(),
                    )
                    .build(),
            )
            .with::<tag::Compound>(
                "option0",
                tag::Compound::builder()
                    .with::<tag::String>("variant", "Some".to_string().into())
                    .with::<tag::String>("value", "hi".to_string().into())
                    .build(),
            )
            .with::<tag::Compound>(
                "option1",
                tag::Compound::builder()
                    .with::<tag::String>("variant", "None".to_string().into())
                    .with::<tag::Compound>("value", tag::Compound::default())
                    .build(),
            )
            .with::<tag::Compound>(
                "map",
                tag::Compound::builder()
                    .with::<tag::Byte>("x", 1.into())
                    .with::<tag::Byte>("y", 0.into())
                    .with::<tag::Byte>("z", 0.into())
                    .build(),
            )
            .with::<tag::Compound>(
                "tuple",
                tag::Compound::builder()
                    .with::<tag::String>("0", "Test".to_string().into())
                    .with::<tag::Byte>("1", 1.into())
                    .with::<tag::Long>("2", 2.into())
                    .build(),
            )
            .with::<tag::Int>("test", 7.into())
            .with::<tag::ByteArray>("vec0", vec![1, 4, 6, 1].into())
            .with::<tag::List>(
                "vec1",
                vec![
                    NBTTag::Short(1.into()),
                    NBTTag::Short(4.into()),
                    NBTTag::Short(6.into()),
                    NBTTag::Short(1.into()),
                ]
                .into(),
            )
            .with::<tag::IntArray>("vec2", vec![1, 4, 6, 1].into())
            .build();
        let output = NBTTag::Compound(output.into());

        let deserialized = deserialize::<ExampleStruct>(&output)
            .unwrap_or_else(|err| panic!("Could not deserialize: {}", err));
        assert_eq!(input, deserialized);

        let serialized =
            serialize(&input).unwrap_or_else(|err| panic!("Could not serialize: {}", err));
        assert_eq!(serialized, output);
    }
}
