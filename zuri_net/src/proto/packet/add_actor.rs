use crate::proto::ints::{VarI64, VarU32, VarU64};
use glam::Vec3;
use zuri_net_derive::proto;

use crate::proto::types::attribute::AttributeValue;
use crate::proto::types::entity_data::{EntityMetadata, EntityProperties};
use crate::proto::types::world::EntityLink;

/// Sent by the server to the client to spawn an entity to the player. It is used for every entity
/// except other players, for which the AddPlayer packet is used.
#[proto]
#[derive(Debug, Clone)]
pub struct AddActor {
    /// The unique ID of the entity. The unique ID is a value that remains consistent across
    /// different sessions of the same world, but most servers simply fill the runtime ID of the
    /// entity out for this field.
    pub entity_unique_id: VarI64,
    /// The runtime ID of the entity. The runtime ID is unique for each world session, and entities
    /// are generally identified in packets using this runtime ID.
    pub entity_runtime_id: VarU64,
    /// The string entity type of the entity. A list of these entities may be found online.
    pub entity_type: String,
    /// The position to spawn the entity on. If the entity is on a distance that the player cannot
    /// see it, the entity will still show up if the player moves closer.
    pub position: Vec3,
    /// The initial velocity the entity spawns with. This velocity will initiate client side
    /// movement of the entity.
    pub velocity: Vec3,
    /// The vertical rotation of the entity. Facing straight forward yields a pitch of 0. Pitch is
    /// measured in degrees.
    pub pitch: f32,
    /// The horizontal rotation of the entity. Yaw is also measured in degrees.
    pub yaw: f32,
    /// The same as yaw, except that it applies specifically to the head of the entity. A different
    /// value for head yaw than yaw means that the entity will have its head turned.
    pub head_yaw: f32,
    /// The same as yaw, except that it applies specifically to the body of the entity. A different
    /// value for body yaw than head yaw means that the entity will have its body turned, although
    /// it is unclear what the difference between body yaw and yaw is.
    pub body_yaw: f32,
    /// A list of attributes that the entity has. It includes attributes such as its health,
    /// movement speed, etc.
    #[len_type(VarU32)]
    pub attributes: Vec<AttributeValue>,
    /// A map of entity metadata, which includes flags and data properties that alter in particular
    /// the way the entity looks. Flags include ones such as 'on fire' and 'sprinting'. The meta
    /// values are indexed by their property key.
    pub entity_metadata: EntityMetadata,
    /// A list of properties that the entity inhibits. These properties define specific attributes
    /// of the entity.
    pub entity_properties: EntityProperties,
    /// A list of entity links that are currently active on the entity. These links alter the way
    /// the entity shows up when first spawned in terms of it shown as riding an entity. Setting
    /// these links is important for new viewers to see the entity is riding another entity.
    #[len_type(VarU32)]
    pub entity_links: Vec<EntityLink>,
}
