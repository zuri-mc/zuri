use zuri_net_derive::proto;

/// Sent by the server (and the client, on development builds) to measure the latency over the
/// entire Minecraft stack, rather than the RakNet latency. It has other usages too, such as the
/// ability to be used as some kind of acknowledgement packet, to know when the client has received
/// a certain other packet.
#[proto]
#[derive(Debug, Clone)]
pub struct NetworkStackLatency {
    /// The timestamp of the network stack latency packet. The client will, if `needs_response` is
    /// set to true, send a NetworkStackLatency packet with this same timestamp packet in response.
    pub timestamp: i64,
    /// Specifies if the sending side of this packet wants a response to the packet, meaning that
    /// the other side should send a NetworkStackLatency packet back.
    pub needs_response: bool,
}
