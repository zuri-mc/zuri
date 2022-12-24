use crate::io::{Reader, Writer};
use crate::packet::Packet;

#[derive(Debug)]
pub struct PlayerInput {
    pub movement: Vec2,
    pub jumping: bool,
    pub sneaking: bool,
}

impl Packet for PlayerInput {
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
