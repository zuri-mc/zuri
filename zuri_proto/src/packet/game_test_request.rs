use glam::IVec3;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

use crate::packet::Packet;
use crate::io::{Reader, Writer};

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum GameTestRequestRotation {
    None,
    Rotate90,
    Rotate180,
    Rotate270,
    Rotate360,
}

#[derive(Debug)]
pub struct GameTestRequest {
    pub max_tests_per_batch: i32,
    pub repetitions: i32,
    pub rotation: GameTestRequestRotation,
    pub stop_on_error: bool,
    pub position: IVec3,
    pub tests_per_row: i32,
    pub name: String,
}

impl Packet for GameTestRequest {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.max_tests_per_batch);
        writer.var_i32(self.repetitions);
        writer.u8(self.rotation.to_u8().unwrap());
        writer.bool(self.stop_on_error);
        writer.block_pos(self.position);
        writer.var_i32(self.tests_per_row);
        writer.string(self.name.as_str());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            max_tests_per_batch: reader.var_i32(),
            repetitions: reader.var_i32(),
            rotation: GameTestRequestRotation::from_u8(reader.u8()).unwrap(),
            stop_on_error: reader.bool(),
            position: reader.block_pos(),
            tests_per_row: reader.var_i32(),
            name: reader.string(),
        }
    }
}
