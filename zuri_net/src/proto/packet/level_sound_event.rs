use crate::proto::ints::VarI32;
use glam::Vec3;
use zuri_net_derive::proto;

use crate::proto::types::sound_event::SoundEvent;

/// Sent by the server to make any kind of built-in sound heard to a player. It is sent to, for
/// example, play a stepping sound or a shear sound. The packet is also sent by the client, in which
/// case it could be forwarded by the server to the other players online. If possible, the packets
/// from the client should be ignored however, and the server should play them on its own accord.
#[proto]
#[derive(Debug, Clone)]
pub struct LevelSoundEvent {
    /// The type of the sound to play. Some of the sound types require additional data, which is set
    /// in the `event_data` field.
    pub sound: SoundEvent,
    /// The position of the sound event. The player will be able to hear the direction of the sound
    /// based on what position is sent here.
    pub position: Vec3,
    /// A packed integer that some sound types use to provide extra data. An example of this is the
    /// note sound, which is composed of a pitch and an instrument type.
    pub extra_data: VarI32,
    /// The string entity type of the entity that emitted the sound, for example
    /// 'minecraft:skeleton'. Some sound types use this entity type for additional data.
    pub entity_type: String,
    /// Specifies if the sound should be that of a baby mob. It is most notably used for parrot
    /// imitations, which will change based on if this field is set to true or not.
    pub baby_mob: bool,
    /// Specifies if the sound should be played relatively or not. If set to true, the sound will
    /// have full volume, regardless of where the position is, whereas if set to false, the sound's
    /// volume will be based on the distance to position.
    pub disable_relative_volume: bool,
}
