use crate::proto::ints::{VarU32, VarU64};
use glam::Vec3;
use uuid::Uuid;
use zuri_net_derive::proto;

use crate::proto::types::ability::AbilityData;
use crate::proto::types::device::Device;
use crate::proto::types::entity_data::{EntityMetadata, EntityProperties};
use crate::proto::types::item::ItemInstance;
use crate::proto::types::world::{EntityLink, GameType};

/// Sent by the server to the client to make a player entity show up client-side. It is one of the
/// few entities that cannot be sent using the AddActor packet.
#[proto]
#[derive(Debug, Clone)]
pub struct AddPlayer {
    /// The UUID of the player. It is the same UUID that the client sent in the Login packet at the
    /// start of the session. A player with this UUID must exist in the player list (built up using
    /// the PlayerList packet), for it to show up in-game.
    pub uuid: Uuid,
    /// The name of the player. This username is the username that will be set as the initial name
    /// tag of the player.
    pub username: String,
    /// The runtime ID of the player. The runtime ID is unique for each world session, and entities
    /// are generally identified in packets using this runtime ID.
    pub entity_runtime_id: VarU64,
    /// An identifier only set for particular platforms when chatting (presumably only for Nintendo
    /// Switch). It is otherwise an empty string, and is used to decide which players are able to
    /// chat with each other.
    pub platform_chat_id: String,
    /// The position to spawn the player on. If the player is on a distance that the viewer cannot
    /// see it, the player will still show up if the viewer moves closer.
    pub position: Vec3,
    /// The initial velocity the player spawns with. This velocity will initiate client side
    /// movement of the player.
    pub velocity: Vec3,
    /// The vertical rotation of the player. Facing straight forward yields a pitch of 0. Pitch is
    /// measured in degrees.
    pub pitch: f32,
    /// The horizontal rotation of the player. Yaw is also measured in degrees.
    pub yaw: f32,
    /// The same as yaw, except that it applies specifically to the head of the player. A different
    /// value for head yaw than yaw means that the player will have its head turned.
    pub head_yaw: f32,
    /// The item that the player is holding. The item is shown to the viewer as soon as the player
    /// itself shows up. Needless to say that this field is rather pointless, as more packets still
    /// must be sent for armour to show up.
    pub held_item: ItemInstance,
    /// The game type of the player. If set to Spectator, the player will not be shown to viewers.
    pub game_type: GameType,
    /// A map of entity metadata, which includes flags and data properties that alter in particular
    /// the way the player looks. Flags include ones such as 'on fire' and 'sprinting'. The meta
    /// values are indexed by their property key.
    pub entity_metadata: EntityMetadata,
    /// A list of properties that the entity inhibits. These properties define specific attributes
    /// of the entity.
    pub entity_properties: EntityProperties,
    /// Represents various data about the abilities of a player, such as ability layers or
    /// permissions.
    pub ability_data: AbilityData,
    /// A list of entity links that are currently active on the player. These links alter the way
    /// the player shows up when first spawned in terms of it shown as riding an entity. Setting
    /// these links is important for new viewers to see the player is riding another entity.
    #[len_type(VarU32)]
    pub entity_links: Vec<EntityLink>,
    /// The device ID set in one of the files found in the storage of the device of the player. It
    /// may be changed freely, so it should not be relied on for anything.
    pub device_id: String,
    /// The build platform/device OS of the player that is about to be added, as sent in the Login
    /// packet.
    pub build_platform: Device,
}
