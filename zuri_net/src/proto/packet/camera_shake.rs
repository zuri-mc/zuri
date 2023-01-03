use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{ToPrimitive, FromPrimitive};

use crate::proto::packet::PacketType;
use crate::proto::io::{Reader, Writer};

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum CameraShakeAction {
    Add,
    Stop,
}

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum CameraShakeType {
    Positional,
    Rotational,
}

/// Sent by the server to make the camera shake client-side. This feature was added for map-making
/// partners.
#[derive(Debug, Clone)]
pub struct CameraShake {
    /// The intensity of the shaking. The client limits this value to 4, so anything higher may not
    /// function, at least as expected.
    pub intensity: f32,
    /// The number of seconds the camera will shake for.
    pub duration: f32,
    /// The type of shake. The different type affects how the shake looks in game.
    pub shake_type: CameraShakeType,
    /// The action to be performed. Currently, the different actions will either add or stop shaking
    /// the camera client-side.
    pub action: CameraShakeAction,
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
