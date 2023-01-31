use num_traits::{FromPrimitive, ToPrimitive};

use crate::compression::Compression;
use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent by the server to update a variety of network settings. These settings modify the way
/// packets are sent over the network stack.
#[derive(Debug, Clone)]
pub struct NetworkSettings {
    /// The minimum size of a packet that is compressed when sent. If the size of a packet is under
    /// this value, it is not compressed. When set to zero, all packets will be left uncompressed.
    pub compression_threshold: u16,
    /// The algorithm that is used to compress packets.
    pub compression_algorithm: Compression,
    /// Regulates whether the client should throttle players when exceeding of the threshold.
    /// Players outside threshold will not be ticked, improving performance on low-end devices.
    pub client_throttle: bool,
    /// The threshold for client throttling. If the number of players exceeds this value, the client
    /// will throttle players.
    pub client_throttle_threshold: u8,
    /// The scalar for client throttling. The scalar is the amount of players that are ticked when
    /// throttling is enabled.
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
            compression_algorithm: Compression::from_u16(reader.u16()).unwrap(),
            client_throttle: reader.bool(),
            client_throttle_threshold: reader.u8(),
            client_throttle_scalar: reader.f32(),
        }
    }
}
