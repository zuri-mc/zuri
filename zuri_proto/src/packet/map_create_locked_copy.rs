use crate::io::{Reader, Writer};
use crate::packet::Packet;

/// Sent by the server to create a locked copy of one map into another map. In vanilla, it is used in the cartography
/// table to create a map that is locked and cannot be modified.
#[derive(Debug)]
pub struct MapCreateLockedCopy {
    /// ID of the map that is being copied. The locked copy will obtain all content that is visible on this map, except
    /// the content will not change.
    pub original_map_id: i64,
    /// ID of the map that holds the locked copy of the map that original_map_id points to. Its contents will be
    /// impossible to change.
    pub new_map_id: i64,
}

impl Packet for MapCreateLockedCopy {
    fn write(&self, writer: &mut Writer) {
        writer.var_i64(self.original_map_id);
        writer.var_i64(self.new_map_id);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            original_map_id: reader.var_i64(),
            new_map_id: reader.var_i64(),
        }
    }
}
