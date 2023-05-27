//! todo
mod deserialize;
mod serialize;

use crate::Value;
use serde::Serialize;
use thiserror::Error;
use crate::serde::serialize::Serializer;

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

#[cfg(test)]
mod tests {
    use crate::serde::serialize;
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

        assert_eq!(serialize(input).unwrap(), output)
    }
}
