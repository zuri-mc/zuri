use zuri_net_derive::proto;

/// Sent by the server to the client to notify the client of a ticking area's loading status.
#[proto]
#[derive(Debug, Clone)]
pub struct TickingAreasLoadStatus {
    /// True if the server is waiting for the area's preload.
    pub preload: bool,
}
