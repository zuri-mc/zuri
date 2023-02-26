use zuri_net_derive::proto;

use crate::proto::io::BlockPos;

/// Sent by the server to play a sound to the client. Some of the sounds may only be started using
/// this packet and must be stopped using the StopSound packet.
#[proto]
#[derive(Debug, Clone)]
pub struct PlaySound {
    /// The name of the sound to play.
    pub sound_name: String,
    /// The position at which the sound was played. Some sounds do not depend on a position, which
    /// will then ignore it, but most of them will play with the direction based on the position
    /// compared to the player's position.
    pub position: BlockPos,
    /// The relative volume of the sound to play. It will be less loud for the player if it is
    /// farther away from the position of the sound.
    pub volume: f32,
    /// The pitch of the sound to play. Some sounds completely ignore this field, whereas others use
    /// it to specify the pitch as the field is intended.
    pub pitch: f32,
}
