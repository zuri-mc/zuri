use glam::IVec3;

use crate::proto::io::{Readable, Reader, Writable, Writer};
use crate::proto::packet::PacketType;
use crate::proto::types::colour::VarRGBA;
use crate::proto::types::map::{MapDecoration, MapTrackedObject, MapUpdateFlag};

/// Sent by the server to the client to update the data of a map shown to the client. It is sent
/// with a combination of flags that specify what data is updated. It may be used to update specific
/// parts of the map only. It is not required to send the full map each time when updating one part.
#[derive(Debug, Clone, Default)]
pub struct ClientBoundMapItemData {
    /// The unique identifier that represents the map that is updated over network. It remains
    /// consistent across sessions.
    pub map_id: i64,
    /// A combination of flags found above that indicate what parts of the map should be updated
    /// client-side.
    pub update_flags: u32,
    /// The dimension of the map that should be updated.
    pub dimension: u8,
    /// Specifies if the map that was updated was a locked map, which may be done using a
    /// cartography table.
    pub locked_map: bool,
    /// The center position of the map being updated.
    pub origin: IVec3,
    /// The scale of the map as it is shown in-game. It is written when any of the map update flags
    /// are set to the update flags field.
    pub scale: u8,
    /// Map IDs that the map updated is included in. This has to do with the scale of the map: Each
    /// map holds its own map ID and all map IDs of maps that include this map and have a bigger
    /// scale. This means that a scale zero map will have five map IDs in this list, whereas a scale
    /// four map will have only one (its own). The actual use of this field remains unknown.
    pub maps_included_in: Vec<i64>,
    /// A list of tracked objects on the map, which may either be entities or blocks. The client
    /// makes sure these tracked objects are actually tracked. (position updated etc.)
    pub tracked_objects: Vec<MapTrackedObject>,
    /// A list of fixed decorations located on the map. The decorations will not change client-side,
    /// unless the server updates them.
    pub decorations: Vec<MapDecoration>,
    /// The width of the texture area that was updated. The width may be a subset of the total width
    /// of the map.
    pub width: i32,
    /// The height of the texture area that was updated. The height may be a subset of the total
    /// height of the map.
    pub height: i32,
    /// The X offset in pixels at which the updated texture area starts. From this X, the updated
    /// texture will extend exactly width pixels to the right.
    pub x_offset: i32,
    /// The Y offset in pixels at which the updated texture area starts. From this Y, the updated
    /// texture will extend exactly height pixels up.
    pub y_offset: i32,
    /// A list of pixel colours for the new texture of the map. It is indexed using [y*height + x].
    pub pixels: Vec<VarRGBA>,
}

impl PacketType for ClientBoundMapItemData {
    fn write(&self, writer: &mut Writer) {
        writer.var_i64(self.map_id);
        writer.var_u32(self.update_flags);
        writer.u8(self.dimension);
        writer.bool(self.locked_map);
        writer.block_pos(self.origin);

        if self.update_flags & MapUpdateFlag::Initialisation.flag() != 0 {
            writer.var_u32(self.maps_included_in.len() as u32);
            self.maps_included_in.iter().for_each(|map_id| {
                writer.var_i64(*map_id);
            });
        }
        if self.update_flags
            & (MapUpdateFlag::Initialisation.flag()
                | MapUpdateFlag::Decoration.flag()
                | MapUpdateFlag::Texture.flag())
            != 0
        {
            writer.u8(self.scale);
        }
        if self.update_flags & MapUpdateFlag::Decoration.flag() != 0 {
            writer.var_u32(self.tracked_objects.len() as u32);
            self.tracked_objects
                .iter()
                .for_each(|tracked_object| tracked_object.write(writer));
            writer.var_u32(self.decorations.len() as u32);
            self.decorations
                .iter()
                .for_each(|decoration| decoration.write(writer));
        }
        if self.update_flags & MapUpdateFlag::Texture.flag() != 0 {
            writer.i32(self.width);
            writer.i32(self.height);
            writer.i32(self.x_offset);
            writer.i32(self.y_offset);
            writer.var_u32(self.pixels.len() as u32);
            self.pixels.iter().for_each(|pixels| pixels.write(writer));
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
        if packet.update_flags
            & (MapUpdateFlag::Initialisation.flag()
                | MapUpdateFlag::Decoration.flag()
                | MapUpdateFlag::Texture.flag())
            != 0
        {
            packet.scale = reader.u8();
        }
        if packet.update_flags & MapUpdateFlag::Decoration.flag() != 0 {
            packet.tracked_objects = (0..reader.var_u32())
                .map(|_| MapTrackedObject::read(reader))
                .collect();
            packet.decorations = (0..reader.var_u32())
                .map(|_| MapDecoration::read(reader))
                .collect();
        }
        if packet.update_flags & MapUpdateFlag::Texture.flag() != 0 {
            packet.width = reader.i32();
            packet.height = reader.i32();
            packet.x_offset = reader.i32();
            packet.y_offset = reader.i32();
            packet.pixels = (0..reader.var_u32())
                .map(|_| VarRGBA::read(reader))
                .collect();
        }

        packet
    }
}
