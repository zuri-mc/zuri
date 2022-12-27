use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::packet::PacketType;
use crate::proto::io::{Reader, Writer};

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum CodeBuilderCategory {
    None,
    Status,
    Instantiation,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum CodeBuilderOperation {
    None,
    Get,
    Set,
    Reset,
}

#[derive(Debug)]
pub struct CodeBuilderSource {
    pub operation: CodeBuilderOperation,
    pub category: CodeBuilderCategory,
    pub value: u8,
}

impl PacketType for CodeBuilderSource {
    fn write(&self, writer: &mut Writer) {
        writer.u8(self.operation.to_u8().unwrap());
        writer.u8(self.category.to_u8().unwrap());
        writer.u8(self.value);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            operation: CodeBuilderOperation::from_u8(reader.u8()).unwrap(),
            category: CodeBuilderCategory::from_u8(reader.u8()).unwrap(),
            value: reader.u8(),
        }
    }
}
