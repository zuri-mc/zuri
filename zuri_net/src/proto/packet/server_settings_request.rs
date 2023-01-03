use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent by the client to request the settings specific to the server. These settings are shown in a
/// separate tab client-side, and have the same structure as a custom form.
#[derive(Debug, Clone)]
pub struct ServerSettingsRequest {}

impl PacketType for ServerSettingsRequest {
    fn write(&self, _: &mut Writer) {}

    fn read(_: &mut Reader) -> Self {
        Self {}
    }
}
