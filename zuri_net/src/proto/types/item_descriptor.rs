use std::fmt::Debug;
use zuri_net_derive::proto;

use crate::encodable_enum;
use crate::proto::ints::VarI32;
use crate::proto::io::{Readable, Reader, Writable, Writer};

encodable_enum!(
    #[derive(Debug, Clone)]
    pub enum ItemDescriptor {
        InvalidDescriptor = 0,
        DefaultDescriptor = 1,
        MoLangDescriptor = 2,
        ItemTagDescriptor = 3,
        DeferredDescriptor = 4,
        ComplexAliasDescriptor = 5,
    }
);

#[derive(Debug, Clone)]
pub struct ItemDescriptorCount {
    pub item_descriptor: ItemDescriptor,
    pub count: VarI32,
}

impl Readable<ItemDescriptorCount> for ItemDescriptorCount {
    fn read(reader: &mut Reader) -> ItemDescriptorCount {
        ItemDescriptorCount {
            item_descriptor: ItemDescriptor::read(reader),
            count: VarI32(reader.var_i32()),
        }
    }
}

impl Writable for ItemDescriptorCount {
    fn write(&self, writer: &mut Writer) {
        self.item_descriptor.write(writer);
        self.count.write(writer);
    }
}

impl Default for ItemDescriptorCount {
    fn default() -> Self {
        Self {
            item_descriptor: ItemDescriptor::InvalidDescriptor(InvalidDescriptor),
            count: VarI32(0),
        }
    }
}

#[proto]
#[derive(Debug, Clone)]
pub struct InvalidDescriptor;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

/// ComplexAliasItemDescriptor represents an item descriptor that uses a single name to identify the
/// item. There is no clear benefit of using this item descriptor and only seem to be used for
/// specific recipes.
#[derive(Debug, Clone)]
pub struct ComplexAliasDescriptor {
    /// The name of the item, which is a name like 'minecraft:stick'.
    name: String,
}

impl ComplexAliasDescriptor {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            name: reader.string(),
        }
    }

    pub fn write(&self, writer: &mut Writer) {
        writer.string(self.name.as_str());
    }
}
