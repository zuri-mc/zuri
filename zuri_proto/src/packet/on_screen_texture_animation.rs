use crate::io::{Reader, Writer};
use crate::packet::Packet;

/// Sent by the server to show a certain animation on the screen of the player. The packet is used, as an example, for
/// when a raid is triggered and when a raid is defeated.
#[derive(Debug)]
pub struct OnScreenTextureAnimation {
    /// Type of the animation to show. The packet provides no further extra data to allow modifying the duration or
    /// other properties of the animation.
    pub animation_type: i32,
}

impl Packet for OnScreenTextureAnimation {
    fn write(&self, writer: &mut Writer) {
        writer.i32(self.animation_type);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            animation_type: reader.i32(),
        }
    }
}
