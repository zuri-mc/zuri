use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{ToPrimitive, FromPrimitive};

use crate::packet::PacketType;
use crate::io::{Reader, Writer};

#[derive(Debug)]
pub struct CameraShake {
    pub intensity: f32,
    pub duration: f32,
    pub shake_type: CameraShakeType,
    pub action: CameraShakeAction,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum CameraShakeAction {
    Add,
    Stop,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum CameraShakeType {
    Positional,
    Rotational,
}

impl PacketType for CameraShake {
    fn write(&self, writer: &mut Writer) {
        writer.f32(self.intensity);
        writer.f32(self.duration);
        writer.u8(self.shake_type.to_u8().unwrap());
        writer.u8(self.action.to_u8().unwrap());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            intensity: reader.f32(),
            duration: reader.f32(),
            shake_type: CameraShakeType::from_u8(reader.u8()).unwrap(),
            action: CameraShakeAction::from_u8(reader.u8()).unwrap(),
        }
    }
}
