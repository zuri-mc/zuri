use zuri_net_derive::packet;

/// Sent by the client to request the settings specific to the server. These settings are shown in a
/// separate tab client-side, and have the same structure as a custom form.
#[packet]
#[derive(Debug, Clone)]
pub struct ServerSettingsRequest;
