use zuri_net_derive::proto;

use crate::compression::Compression;

/// Sent by the server to update a variety of network settings. These settings modify the way
/// packets are sent over the network stack.
#[proto]
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
