//! Implementations for type conversions from and to [Value] using [From] and [TryFrom].
use crate::Value;
use std::collections::HashMap;

macro_rules! impl_conv_simple {
    ($typ:ty, $enum_variant:path) => {
        impl TryFrom<Value> for $typ {
            type Error = Value;

            fn try_from(value: Value) -> Result<Self, Self::Error> {
                if let $enum_variant(v) = value {
                    Ok(v)
                } else {
                    Err(value)
                }
            }
        }

        impl From<$typ> for Value {
            fn from(value: $typ) -> Self {
                $enum_variant(value)
            }
        }
    };
    ($(($typ:ty, $enum_variant:path)$(,)?)*) => {
        $(impl_conv_simple!($typ, $enum_variant);)*
    };
}

impl_conv_simple!(
    (u8, Value::Byte),
    (i16, Value::Short),
    (i32, Value::Int),
    (i64, Value::Long),
    (f32, Value::Float),
    (f64, Value::Double),
    (String, Value::String),
    (HashMap<String, Value>, Value::Compound),
    (Vec<Value>, Value::List),
    (Vec<u8>, Value::ByteArray),
    (Vec<i32>, Value::IntArray),
    (Vec<i64>, Value::LongArray),
);
