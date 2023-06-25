use crate::client::NetworkSet;
use crate::entity::{despawn_entity_system, spawn_entity_system};
use bevy::prelude::*;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use zuri_net::proto::ints::{VarI64, VarU64};

pub(super) struct EntityManagerPlugin;

impl Plugin for EntityManagerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EntityManager::default()).add_systems(
            (
                apply_system_buffers.after(despawn_entity_system),
                entity_remove_system,
                apply_system_buffers.after(spawn_entity_system),
                entity_add_system,
            )
                .chain()
                .in_base_set(NetworkSet::Process),
        );
    }
}

/// Manages and tracks minecraft entities sent by the server.
///
/// Automatically tracks entities with the [IdentifiableEntity] component.
#[derive(Default, Resource)]
pub struct EntityManager {
    /// Maps minecraft entity runtime identifiers to an ECS entity.
    runtime_id_map: HashMap<RuntimeId, (UniqueId, Entity)>,
    /// Maps minecraft entity unique identifiers to an ECS entity.
    unique_id_map: HashMap<UniqueId, (RuntimeId, Entity)>,
    /// Maps ECS entities to their runtime id. Used for efficiently removing entities.
    entity_to_rid: HashMap<Entity, RuntimeId>,
}

impl EntityManager {
    /// Retrieves an entity by its [RuntimeId].
    pub fn entity_by_rid(&self, id: impl Into<RuntimeId>) -> Option<Entity> {
        self.runtime_id_map.get(&id.into()).map(|v| v.1)
    }

    /// Retrieves an entity by its [UniqueId].
    pub fn entity_by_uid(&self, id: impl Into<UniqueId>) -> Option<Entity> {
        self.unique_id_map.get(&id.into()).map(|v| v.1)
    }
}

/// A component for all entities that should be managed by the [EntityManager].
///
/// Giving or removing this component will add or remove the entity from the [EntityManager].
#[derive(Component, Copy, Clone)]
pub struct IdentifiableEntity(RuntimeId, UniqueId);

impl IdentifiableEntity {
    /// Creates a new [IdentifiableEntity] component.
    pub fn new(runtime_id: impl Into<RuntimeId>, unique_id: impl Into<UniqueId>) -> Self {
        Self(runtime_id.into(), unique_id.into())
    }

    /// Returns the entity's runtime id.
    pub fn runtime_id(&self) -> RuntimeId {
        self.0
    }

    /// Returns the entity's unique id.
    pub fn unique_id(&self) -> UniqueId {
        self.1
    }
}

/// A unique runtime identifier for an entity at runtime. It is defined by the server and does not
/// persist.
#[derive(Copy, Clone, Debug, Ord, Eq, Hash)]
pub struct RuntimeId(pub u64);

impl Display for RuntimeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <u64 as Display>::fmt(&self.0, f)
    }
}

impl<T: Into<RuntimeId> + Copy> PartialEq<T> for RuntimeId {
    fn eq(&self, other: &T) -> bool {
        self.0 == other.clone().into().0
    }
}

impl<T: Into<RuntimeId> + Copy> PartialOrd<T> for RuntimeId {
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        self.0.partial_cmp(&other.clone().into().0)
    }
}

impl From<u64> for RuntimeId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<RuntimeId> for u64 {
    fn from(value: RuntimeId) -> Self {
        value.0
    }
}

impl From<VarU64> for RuntimeId {
    fn from(value: VarU64) -> Self {
        Self(value.0)
    }
}

impl From<RuntimeId> for VarU64 {
    fn from(value: RuntimeId) -> Self {
        VarU64(value.0)
    }
}

/// The unique identifier for a minecraft entity. It is supposed to persist, but in reality it does
/// not need to.
#[derive(Copy, Clone, Debug, Ord, Eq, Hash)]
pub struct UniqueId(i64);

impl Display for UniqueId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <i64 as Display>::fmt(&self.0, f)
    }
}

impl<T: Into<UniqueId> + Copy> PartialEq<T> for UniqueId {
    fn eq(&self, other: &T) -> bool {
        self.0 == other.clone().into().0
    }
}

impl<T: Into<UniqueId> + Copy> PartialOrd<T> for UniqueId {
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        self.0.partial_cmp(&other.clone().into().0)
    }
}

impl From<i64> for UniqueId {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl From<UniqueId> for i64 {
    fn from(value: UniqueId) -> Self {
        value.0
    }
}

impl From<VarI64> for UniqueId {
    fn from(value: VarI64) -> Self {
        Self(value.0)
    }
}

impl From<UniqueId> for VarI64 {
    fn from(value: UniqueId) -> Self {
        VarI64(value.0)
    }
}

/// Adds entities to the entity manager when they are added in the world with a [IdentifiableEntity]
/// component.
fn entity_add_system(
    mut commands: Commands,
    mut entities: ResMut<EntityManager>,
    new_entities: Query<(Entity, &IdentifiableEntity), Added<IdentifiableEntity>>,
) {
    for (ecs_entity, mc_entity) in new_entities.iter() {
        if let Some(prev) = entities
            .runtime_id_map
            .insert(mc_entity.runtime_id(), (mc_entity.unique_id(), ecs_entity))
        {
            error!(
                "Overriding entity with runtime id {} (previously {:?})",
                mc_entity.runtime_id(),
                prev.1,
            );
            commands.entity(prev.1).despawn();
            if prev.0 != mc_entity.unique_id() {
                entities.unique_id_map.remove(&prev.0);
            }
            continue;
        }

        if let Some(prev) = entities
            .unique_id_map
            .insert(mc_entity.unique_id(), (mc_entity.runtime_id(), ecs_entity))
        {
            error!(
                "Overriding entity with unique id {} (previously {:?})",
                mc_entity.unique_id(),
                prev.1,
            );
            commands.entity(prev.1).despawn();
            if prev.0 != mc_entity.runtime_id() {
                entities.runtime_id_map.remove(&prev.0);
            }
            continue;
        }

        entities
            .entity_to_rid
            .insert(ecs_entity, mc_entity.runtime_id());

        info!(
            "Now tracking entity with runtime id {}",
            mc_entity.runtime_id()
        );
    }
}

/// Removes entities from the entity manager if they or their [IdentifiableEntity] component get
/// removed.
fn entity_remove_system(
    mut entities: ResMut<EntityManager>,
    mut removed: RemovedComponents<IdentifiableEntity>,
) {
    for entity in removed.iter() {
        let rid = entities.entity_to_rid.get(&entity).cloned();
        if rid.is_none() {
            continue;
        }
        let rid = rid.unwrap();
        let uid = entities
            .runtime_id_map
            .remove(&rid)
            .expect("Entity did not have unique id")
            .0;
        entities.unique_id_map.remove(&uid);
        entities.entity_to_rid.remove(&entity);

        info!("Stopped tracking entity with runtime id {}", rid);
    }
}
