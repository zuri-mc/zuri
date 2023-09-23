use zuri_net_derive::proto;

/// Sent by the server to refresh the player's entitlements.
#[proto]
#[derive(Debug, Clone)]
pub struct RefreshEntitlements;
