use glam::IVec3;
use crate::io::{Reader, Writer};
use crate::packet::Packet;
use crate::types::colour::RGBA;
use crate::types::map::{MapDecoration, MapTrackedObject, MapUpdateFlag};

#[derive(Debug, Default)]
pub struct ClientBoundMapItemData {
    pub map_id: i64,
    pub update_flags: u32,
    pub dimension: u8,
    pub locked_map: bool,
    pub origin: IVec3,
    pub scale: u8,
    pub maps_included_in: Vec<i64>,
    pub tracked_objects: Vec<MapTrackedObject>,
    pub decorations: Vec<MapDecoration>,
    pub width: i32,
    pub height: i32,
    pub x_offset: i32,
    pub y_offset: i32,
    pub pixels: Vec<RGBA>,
}

impl Packet for ClientBoundMapItemData {
    fn write(&self, writer: &mut Writer) {
        writer.var_i64(self.map_id);
        writer.var_u32(self.update_flags);
        writer.u8(self.dimension);
        writer.bool(self.locked_map);
        writer.block_pos(self.origin);

        if self.update_flags & MapUpdateFlag::Initialisation.flag() != 0 {
            writer.var_u32(self.maps_included_in.len() as u32);
            self.maps_included_in.iter().for_each(|map_id| { writer.var_i64(*map_id); });
        }
        if self.update_flags & (MapUpdateFlag::Initialisation.flag() | MapUpdateFlag::Decoration.flag() | MapUpdateFlag::Texture.flag()) != 0 {
            writer.u8(self.scale);
        }
        if self.update_flags & MapUpdateFlag::Decoration.flag() != 0 {
            writer.var_u32(self.tracked_objects.len() as u32);
            self.tracked_objects.iter().for_each(|tracked_object| tracked_object.write(writer));
            writer.var_u32(self.decorations.len() as u32);
            self.decorations.iter().for_each(|decoration| decoration.write(writer));
        }
        if self.update_flags & MapUpdateFlag::Texture.flag() != 0 {
            writer.i32(self.width);
            writer.i32(self.height);
            writer.i32(self.x_offset);
            writer.i32(self.y_offset);
            writer.var_u32(self.pixels.len() as u32);
            self.pixels.iter().for_each(|pixels| pixels.write_var(writer));
        }
    }

    fn read(reader: &mut Reader) -> Self {
        let mut packet = Self {
            map_id: reader.var_i64(),
            update_flags: reader.var_u32(),
            dimension: reader.u8(),
            locked_map: reader.bool(),
            origin: reader.block_pos(),
            ..Default::default()
        };
        if packet.update_flags & MapUpdateFlag::Initialisation.flag() != 0 {
            packet.maps_included_in = (0..reader.var_u32()).map(|_| reader.var_i64()).collect();
        }
        if packet.update_flags & (MapUpdateFlag::Initialisation.flag() | MapUpdateFlag::Decoration.flag() | MapUpdateFlag::Texture.flag()) != 0 {
            packet.scale = reader.u8();
        }
        if packet.update_flags & MapUpdateFlag::Decoration.flag() != 0 {
            packet.tracked_objects = (0..reader.var_u32()).map(|_| MapTrackedObject::read(reader)).collect();
            packet.decorations = (0..reader.var_u32()).map(|_| MapDecoration::read(reader)).collect();
        }
        if packet.update_flags & MapUpdateFlag::Texture.flag() != 0 {
            packet.width = reader.i32();
            packet.height = reader.i32();
            packet.x_offset = reader.i32();
            packet.y_offset = reader.i32();
            packet.pixels = (0..reader.var_u32()).map(|_| RGBA::read_var(reader)).collect();
        }

        packet
    }
}
