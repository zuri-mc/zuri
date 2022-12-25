use std::fmt::Debug;

use crate::encodable_enum;
use crate::io::{Reader, Writer};

encodable_enum!(
    #[derive(Debug)]
    pub enum ItemDescriptor {
        InvalidDescriptor = 0,
        DefaultDescriptor = 1,
        MoLangDescriptor = 2,
        ItemTagDescriptor = 3,
        DeferredDescriptor = 4,
    }
);

#[derive(Debug)]
pub struct ItemDescriptorCount {
    pub item_descriptor: ItemDescriptor,
    pub count: i32,
}

impl ItemDescriptorCount {
    pub fn write(&self, writer: &mut Writer) {
        self.item_descriptor.write(writer);
        writer.var_i32(self.count);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            item_descriptor: ItemDescriptor::read(reader),
            count: reader.var_i32(),
        }
    }
}

impl Default for ItemDescriptorCount {
    fn default() -> Self {
        Self {
            item_descriptor: ItemDescriptor::InvalidDescriptor(InvalidDescriptor),
            count: 0,
        }
    }
}

#[derive(Debug)]
pub struct InvalidDescriptor;

impl InvalidDescriptor {
    pub fn read(_: &mut Reader) -> Self {
        Self {}
    }

    pub fn write(&self, _: &mut Writer) {}
}

#[derive(Debug)]
pub struct DefaultDescriptor {
    network_id: i16,
    metadata: i16,
}

impl DefaultDescriptor {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            network_id: reader.i16(),
            metadata: reader.i16(),
        }
    }

    pub fn write(&self, writer: &mut Writer) {
        writer.i16(self.network_id);
        writer.i16(self.metadata);
    }
}

#[derive(Debug)]
pub struct MoLangDescriptor {
    expression: String,
    version: u8,
}

impl MoLangDescriptor {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            expression: reader.string(),
            version: reader.u8(),
        }
    }

    pub fn write(&self, writer: &mut Writer) {
        writer.string(self.expression.as_str());
        writer.u8(self.version);
    }
}

#[derive(Debug)]
pub struct ItemTagDescriptor {
    tag: String,
}

impl ItemTagDescriptor {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            tag: reader.string(),
        }
    }

    pub fn write(&self, writer: &mut Writer) {
        writer.string(self.tag.as_str());
    }
}

#[derive(Debug)]
pub struct DeferredDescriptor {
    name: String,
    metadata: i16,
}

impl DeferredDescriptor {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            name: reader.string(),
            metadata: reader.i16(),
        }
    }

    pub fn write(&self, writer: &mut Writer) {
        writer.string(self.name.as_str());
        writer.i16(self.metadata);
    }
}
