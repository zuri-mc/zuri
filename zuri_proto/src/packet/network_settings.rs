use num_derive::{FromPrimitive, ToPrimitive};
use crate::io::{Reader, Writer};
use crate::packet::Packet;

#[derive(Debug, Copy, Clone, FromPrimitive, ToPrimitive)]
pub enum CompressionType {
    Flate,
    Snappy,
}

#[derive(Debug)]
pub struct NetworkSettings {
    pub compression_threshold: u16,
    pub compression_algorithm: CompressionType,
    pub client_throttle: bool,
    pub client_throttle_threshold: u8,
    pub client_throttle_scalar: f32,
}

impl Packet for NetworkSettings {
    fn write(&self, writer: &mut Writer) {
        writer.u16(self.compression_threshold);
        writer.u16(num::ToPrimitive::to_u16(&self.compression_algorithm).unwrap());
        writer.bool(self.client_throttle);
        writer.u8(self.client_throttle_threshold);
        writer.f32(self.client_throttle_scalar);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            compression_threshold: reader.u16(),
            compression_algorithm: num::FromPrimitive::from_u16(reader.u16()).unwrap(),
            client_throttle: reader.bool(),
            client_throttle_threshold: reader.u8(),
            client_throttle_scalar: reader.f32(),
        }
    }
}
