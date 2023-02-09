use zuri_net_derive::proto;

/// Sent by the server to stop a sound playing to the player, such as a playing music disk track or
/// other long-lasting sounds.
#[proto]
#[derive(Debug, Clone)]
pub struct StopSound {
    /// The name of the sound that should be stopped from playing. If no sound with this name is
    /// currently active, the packet is ignored.
    pub sound_name: String,
    /// Specifies if all sounds currently playing to the player should be stopped. If set to true,
    /// the `sound_name` field may be left empty.
    pub stop_all: bool,
}
