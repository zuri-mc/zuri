use glam::IVec3;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use zuri_net_derive::packet;

use crate::proto::io::{Reader, Writer};
use crate::proto::types::colour::VarRGBA;

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum MapObjectType {
    Entity,
    Block,
}

#[derive(Clone, Copy, Debug, FromPrimitive, ToPrimitive)]
pub enum MapUpdateFlag {
    Texture,
    Decoration,
    Initialisation,
}

impl MapUpdateFlag {
    pub fn flag(&self) -> u32 {
        1 << (*self as u32)
    }
}

#[packet]
#[derive(Debug, Clone)]
pub struct MapDecoration {
    pub decoration_type: u8,
    pub rotation: u8,
    pub x: u8,
    pub y: u8,
    pub label: String,
    pub colour: VarRGBA,
}

#[derive(Debug, Clone)]
pub struct MapTrackedObject {
    pub object_type: MapObjectType,
    pub entity_unique_id: i64,
    pub block_position: IVec3,
}

impl MapTrackedObject {
    pub fn write(&self, writer: &mut Writer) {
        writer.i32(self.object_type.to_i32().unwrap());
        writer.i64(self.entity_unique_id);
        writer.u_block_pos(self.block_position);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            object_type: MapObjectType::from_i32(reader.i32()).unwrap(),
            entity_unique_id: reader.i64(),
            block_position: reader.u_block_pos(),
        }
    }
}
