use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::packet::PacketType;
use crate::proto::io::{Reader, Writer};

#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum CompressionType {
    Deflate,
    Snappy,
}

#[derive(Debug, Clone)]
pub struct NetworkSettings {
    pub compression_threshold: u16,
    pub compression_algorithm: CompressionType,
    pub client_throttle: bool,
    pub client_throttle_threshold: u8,
    pub client_throttle_scalar: f32,
}

impl PacketType for NetworkSettings {
    fn write(&self, writer: &mut Writer) {
        writer.u16(self.compression_threshold);
        writer.u16(self.compression_algorithm.to_u16().unwrap());
        writer.bool(self.client_throttle);
        writer.u8(self.client_throttle_threshold);
        writer.f32(self.client_throttle_scalar);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            compression_threshold: reader.u16(),
            compression_algorithm: CompressionType::from_u16(reader.u16()).unwrap(),
            client_throttle: reader.bool(),
            client_throttle_threshold: reader.u8(),
            client_throttle_scalar: reader.f32(),
        }
    }
}
