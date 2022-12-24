use std::fmt::Debug;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

use crate::io::{Reader, Writer};

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum ItemDescriptorType {
    Invalid,
    Default,
    MoLang,
    ItemTag,
    Deferred,
}

pub trait ItemDescriptor: Debug {
    fn write(&self, writer: &mut Writer);
    fn descriptor_type(&self) -> ItemDescriptorType;
}

#[derive(Debug)]
pub struct ItemDescriptorCount {
    pub item_descriptor: Box<dyn ItemDescriptor>,
    pub count: i32,
}

impl ItemDescriptorCount {
    pub fn write(&self, writer: &mut Writer) {
        writer.u8(self.item_descriptor.descriptor_type().to_u8().unwrap());
        self.item_descriptor.write(writer);
        writer.var_i32(self.count);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            item_descriptor: match ItemDescriptorType::from_u8(reader.u8()).unwrap() {
                ItemDescriptorType::Invalid => Box::from(InvalidItemDescriptor::read(reader)),
                ItemDescriptorType::Default => Box::from(DefaultItemDescriptor::read(reader)),
                ItemDescriptorType::MoLang => Box::from(MoLangItemDescriptor::read(reader)),
                ItemDescriptorType::ItemTag => Box::from(ItemTagItemDescriptor::read(reader)),
                ItemDescriptorType::Deferred => Box::from(DeferredItemDescriptor::read(reader)),
            },
            count: reader.var_i32(),
        }
    }
}

impl Default for ItemDescriptorCount {
    fn default() -> Self {
        Self {
            item_descriptor: Box::from(InvalidItemDescriptor {}),
            count: 0,
        }
    }
}

#[derive(Debug)]
pub struct InvalidItemDescriptor {}

impl InvalidItemDescriptor {
    pub fn read(_: &mut Reader) -> Self {
        Self {}
    }
}

impl ItemDescriptor for InvalidItemDescriptor {
    fn write(&self, _: &mut Writer) {}

    fn descriptor_type(&self) -> ItemDescriptorType {
        ItemDescriptorType::Invalid
    }
}

#[derive(Debug)]
pub struct DefaultItemDescriptor {
    network_id: i16,
    metadata: i16,
}

impl DefaultItemDescriptor {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            network_id: reader.i16(),
            metadata: reader.i16(),
        }
    }
}

impl ItemDescriptor for DefaultItemDescriptor {
    fn write(&self, writer: &mut Writer) {
        writer.i16(self.network_id);
        writer.i16(self.metadata);
    }

    fn descriptor_type(&self) -> ItemDescriptorType {
        ItemDescriptorType::Deferred
    }
}

#[derive(Debug)]
pub struct MoLangItemDescriptor {
    expression: String,
    version: u8,
}

impl MoLangItemDescriptor {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            expression: reader.string(),
            version: reader.u8(),
        }
    }
}

impl ItemDescriptor for MoLangItemDescriptor {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.expression.as_str());
        writer.u8(self.version);
    }

    fn descriptor_type(&self) -> ItemDescriptorType {
        ItemDescriptorType::MoLang
    }
}

#[derive(Debug)]
pub struct ItemTagItemDescriptor {
    tag: String,
}

impl ItemTagItemDescriptor {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            tag: reader.string(),
        }
    }
}

impl ItemDescriptor for ItemTagItemDescriptor {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.tag.as_str());
    }

    fn descriptor_type(&self) -> ItemDescriptorType {
        ItemDescriptorType::Deferred
    }
}

#[derive(Debug)]
pub struct DeferredItemDescriptor {
    name: String,
    metadata: i16,
}

impl DeferredItemDescriptor {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            name: reader.string(),
            metadata: reader.i16(),
        }
    }
}

impl ItemDescriptor for DeferredItemDescriptor {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.name.as_str());
        writer.i16(self.metadata);
    }

    fn descriptor_type(&self) -> ItemDescriptorType {
        ItemDescriptorType::Deferred
    }
}
