use zuri_net_derive::proto;

/// Sent by the client to the server at the start of the game. It is sent to let the server know if
/// it supports the client-side blob cache. Clients such as Nintendo Switch do not support the
/// cache, and attempting to use it anyway will fail.
#[proto]
#[derive(Debug, Clone)]
pub struct ClientCacheStatus {
    /// Specifies if the blob cache is enabled. If false, the server should not attempt to use the
    /// blob cache. If true, it may do so, but it may also choose not to use it.
    pub enabled: bool,
}
