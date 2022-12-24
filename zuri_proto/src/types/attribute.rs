use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{ToPrimitive, FromPrimitive};
use crate::io::{Reader, Writer};

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum AttributeModifierOperand {
    Min,
    Max,
    Current,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum AttributeModifierOperation {
    Addition,
    MultiplyBase,
    MultiplyTotal,
    Cap,
}

#[derive(Debug, Default)]
pub struct Attribute {
    pub value: AttributeValue,
    pub default: f32,
    pub modifiers: Vec<AttributeModifier>,
}

impl Attribute {
    pub fn write(&self, writer: &mut Writer) {
        writer.f32(self.value.min);
        writer.f32(self.value.max);
        writer.f32(self.value.value);
        writer.f32(self.default);
        writer.string(self.value.name.as_str());
        writer.var_u32(self.modifiers.len() as u32);
        self.modifiers.iter().for_each(|modifier| modifier.write(writer));
    }

    pub fn read(reader: &mut Reader) -> Self {
        let mut attribute = Self::default();
        attribute.value = AttributeValue {
            min: reader.f32(),
            max: reader.f32(),
            value: reader.f32(),
            ..Default::default()
        };
        attribute.default = reader.f32();
        attribute.value.name = reader.string();
        attribute.modifiers = (0..reader.var_u32()).map(|_| AttributeModifier::read(reader)).collect();

        attribute
    }
}

#[derive(Debug)]
pub struct AttributeModifier {
    pub id: String,
    pub name: String,
    pub amount: f32,
    pub operation: AttributeModifierOperation,
    pub operand: AttributeModifierOperand,
    pub serializable: bool,
}

impl AttributeModifier {
    pub fn write(&self, writer: &mut Writer) {
        writer.string(self.id.as_str());
        writer.string(self.name.as_str());
        writer.f32(self.amount);
        writer.i32(self.operation.to_i32().unwrap());
        writer.i32(self.operand.to_i32().unwrap());
        writer.bool(self.serializable);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            id: reader.string(),
            name: reader.string(),
            amount: reader.f32(),
            operation: AttributeModifierOperation::from_i32(reader.i32()).unwrap(),
            operand: AttributeModifierOperand::from_i32(reader.i32()).unwrap(),
            serializable: reader.bool(),
        }
    }
}

#[derive(Debug, Default)]
pub struct AttributeValue {
    pub name: String,
    pub min: f32,
    pub value: f32,
    pub max: f32,
}

impl AttributeValue {
    pub fn write(&self, writer: &mut Writer) {
        writer.string(self.name.as_str());
        writer.f32(self.min);
        writer.f32(self.value);
        writer.f32(self.max);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            name: reader.string(),
            min: reader.f32(),
            value: reader.f32(),
            max: reader.f32(),
        }
    }
}
