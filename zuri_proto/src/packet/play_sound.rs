use crate::io::{Reader, Writer};
use crate::packet::Packet;

#[derive(Debug)]
pub struct PlaySound {
    pub sound_name: String,
    pub position: Vec3,
    pub volume: f32,
    pub pitch: f32,
}

impl Packet for PlaySound {
    fn write(&self, writer: &mut Writer) {
        let block_pos = BlockPos { x: self.position.x as i32 * 8, y: self.position.y as i32 * 8, z: self.position.z as i32 * 8 };
        writer.string(self.sound_name.as_str());
        writer.block_pos(block_pos);
        writer.f32(self.volume);
        writer.f32(self.pitch);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            sound_name: reader.string(),
            position: {
                let block_pos = reader.block_pos();
                Vec3 { x: block_pos.x as f32 / 8.0, y: block_pos.y as f32 / 8.0, z: block_pos.z as f32 / 8.0 }
            },
            volume: reader.f32(),
            pitch: reader.f32(),
        }
    }
}
