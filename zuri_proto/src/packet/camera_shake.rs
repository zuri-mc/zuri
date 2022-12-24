#[derive(Debug)]
pub struct CameraShake {
    pub intensity: f32,
    pub duration: f32,
    pub shake_type: CameraShakeType,
    pub action: CameraShakeAction,
}

impl Packet for CameraShake {
    fn write(&self, writer: &mut Writer) {
        writer.f32(self.intensity);
        writer.f32(self.duration);
        writer.u8(num::ToPrimitive::to_u8(&self.shake_type).unwrap());
        writer.u8(num::ToPrimitive::to_u8(&self.action).unwrap());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            intensity: reader.f32(),
            duration: reader.f32(),
            shake_type: num::FromPrimitive::from_u8(reader.u8()).unwrap(),
            action: num::FromPrimitive::from_u8(reader.u8()).unwrap(),
        }
    }
}
