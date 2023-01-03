use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent by the server to use an Education Edition camera on a player. It produces an image
/// client-side.
#[derive(Debug, Clone)]
pub struct Camera {
    /// The unique ID of the camera entity from which the picture was taken.
    pub camera_entity_unique_id: i64,
    /// The unique ID of the target player. The unique ID is a value that remains consistent across
    /// different sessions of the same world, but most servers simply fill the runtime ID of the
    /// player out for this field.
    pub target_player_unique_id: i64,
}

impl PacketType for Camera {
    fn write(&self, writer: &mut Writer) {
        writer.var_i64(self.camera_entity_unique_id);
        writer.var_i64(self.target_player_unique_id);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            camera_entity_unique_id: reader.var_i64(),
            target_player_unique_id: reader.var_i64(),
        }
    }
}
