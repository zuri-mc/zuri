use zuri_net_derive::packet;

/// Sent by the server to send a 'simple event' to the client, meaning an event without any
/// additional event data. The event is typically used by the client for telemetry.
#[packet]
#[derive(Debug, Clone)]
pub struct SimpleEvent {
    /// The type of the event to be called.
    pub event_type: SimpleEventType,
}

#[packet(i16)]
#[derive(Debug, Clone)]
pub enum SimpleEventType {
    None,
    CommandsEnabled,
    CommandsDisabled,
    UnlockWorldTemplateSettings,
}
