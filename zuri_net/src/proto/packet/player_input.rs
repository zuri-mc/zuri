use glam::Vec2;
use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

#[derive(Debug, Clone)]
pub struct PlayerInput {
    pub movement: Vec2,
    pub jumping: bool,
    pub sneaking: bool,
}

impl PacketType for PlayerInput {
    fn write(&self, writer: &mut Writer) {
        writer.vec2(self.movement);
        writer.bool(self.jumping);
        writer.bool(self.sneaking);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            movement: reader.vec2(),
            jumping: reader.bool(),
            sneaking: reader.bool(),
        }
    }
}
