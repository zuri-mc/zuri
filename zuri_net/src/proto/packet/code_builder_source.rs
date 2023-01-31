use bytes::Bytes;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum CodeBuilderCategory {
    None,
    Status,
    Instantiation,
}

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum CodeBuilderOperation {
    None,
    Get,
    Set,
    Reset,
}

/// Education Edition packet sent by the client to run an operation with a code builder.
#[derive(Debug, Clone)]
pub struct CodeBuilderSource {
    /// The operation to be performed.
    pub operation: CodeBuilderOperation,
    /// The category in which the operation falls under.
    pub category: CodeBuilderCategory,
    /// Extra data about the operation performed. It is always empty unless the operation is set.
    pub value: Bytes,
}

impl PacketType for CodeBuilderSource {
    fn write(&self, writer: &mut Writer) {
        writer.u8(self.operation.to_u8().unwrap());
        writer.u8(self.category.to_u8().unwrap());
        writer.byte_slice(&self.value);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            operation: CodeBuilderOperation::from_u8(reader.u8()).unwrap(),
            category: CodeBuilderCategory::from_u8(reader.u8()).unwrap(),
            value: reader.byte_slice(),
        }
    }
}
