use num_derive::{FromPrimitive, ToPrimitive};

use crate::io::{Reader, Writer};
use crate::packet::Packet;

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

impl Packet for CodeBuilderSource {
    fn write(&self, writer: &mut Writer) {
        writer.u8(num::ToPrimitive::to_u8(&self.operation).unwrap());
        writer.u8(num::ToPrimitive::to_u8(&self.category).unwrap());
        writer.u8(self.value);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            operation: num::FromPrimitive::from_u8(reader.u8()).unwrap(),
            category: num::FromPrimitive::from_u8(reader.u8()).unwrap(),
            value: reader.u8(),
        }
    }
}
