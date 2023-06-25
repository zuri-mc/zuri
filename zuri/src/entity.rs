mod manager;

pub use manager::{EntityManager, RuntimeId, UniqueId};

use crate::entity::manager::{EntityManagerPlugin, IdentifiableEntity};
use crate::player;
use bevy::prelude::*;
use zuri_net::proto::packet::add_actor::AddActor;
use zuri_net::proto::packet::add_player::AddPlayer;
use zuri_net::proto::packet::move_actor_absolute::MoveActorAbsolute;
use zuri_net::proto::packet::remove_actor::RemoveActor;
use zuri_net::proto::packet::start_game::StartGame;

/// Manages entities. Entities here refers to minecraft entities, not all ECS entities.
pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_stage_after(
            CoreStage::PreUpdate,
            EntityStage::Spawn,
            SystemStage::parallel(),
        )
        .add_stage_after(
            EntityStage::Spawn,
            EntityStage::Process,
            SystemStage::parallel(),
        )
        .add_stage_after(
            EntityStage::Process,
            EntityStage::Despawn,
            SystemStage::parallel(),
        )
        .add_plugin(EntityManagerPlugin)
        .add_system_to_stage(EntityStage::Spawn, init_player_system)
        .add_system_to_stage(EntityStage::Process, move_entity_system)
        .add_system_to_stage(EntityStage::Spawn, spawn_entity_system)
        .add_system_to_stage(EntityStage::Despawn, despawn_entity_system);
    }
}

#[derive(StageLabel, Copy, Clone)]
pub enum EntityStage {
    Spawn,
    Process,
    Despawn,
}

/// Basic components required by every entity.
#[derive(Bundle)]
pub struct BaseEntityBundle {
    #[bundle]
    pub transform: TransformBundle,
}

/// A component for an entity with a head that has separate rotation from its body.
#[derive(Component, Default)]
pub struct Head {
    /// The rotation of the head relative to the body.
    pub rot: Quat,
    pub eye_height: f32,
}

/// Initialise the player's [IdentifiableEntity] component on [StartGame].
fn init_player_system(
    mut commands: Commands,
    mut events: EventReader<StartGame>,
    query: Query<Entity, With<player::Local>>,
) {
    if let Some(event) = events.iter().next() {
        let local_player = query.single();

        commands
            .entity(local_player)
            .insert(IdentifiableEntity::new(
                event.entity_runtime_id,
                event.entity_unique_id,
            ));
    }
}

/// Adds entities spawned by the server. For now, they are shown as a simple capsule mesh/
fn spawn_entity_system(
    mut commands: Commands,

    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,

    mut pks: EventReader<AddPlayer>,
    mut pks2: EventReader<AddActor>,
) {
    let spawn_func = &mut |ident: IdentifiableEntity, position: Vec3, color: Color| {
        // We spawn a capsule for now.
        let mut mat = StandardMaterial::from(color);
        mat.reflectance = 0.01;
        mat.metallic = 0.;

        commands
            .spawn(PbrBundle {
                mesh: meshes.add(shape::Capsule { ..default() }.into()),
                material: mats.add(mat).into(),
                transform: Transform::from_xyz(position.x, position.y, position.z),
                global_transform: Default::default(),
                visibility: Default::default(),
                computed_visibility: Default::default(),
            })
            .insert(ident);
    };

    for pk in pks.iter() {
        spawn_func(
            IdentifiableEntity::new(pk.entity_runtime_id, pk.ability_data.entity_unique_id),
            pk.position,
            Color::RED,
        );
    }
    for pk in pks2.iter() {
        spawn_func(
            IdentifiableEntity::new(pk.entity_runtime_id, pk.entity_unique_id),
            pk.position,
            Color::BLUE,
        );
    }
}

/// Updates the position of entities on the server.
fn move_entity_system(
    manager: Res<EntityManager>,
    mut query: Query<&mut Transform>,
    mut pks: EventReader<MoveActorAbsolute>,
) {
    for pk in pks.iter() {
        let entity = manager.entity_by_rid(pk.entity_runtime_id);
        if entity.is_none() {
            error!(
                "Received MoveActorAbsolute for unknown entity with runtime id `{}`",
                pk.entity_runtime_id
            );
            continue;
        }

        if let Ok(mut transform) = query.get_mut(entity.unwrap()) {
            transform.translation = pk.position;
        }
    }
}

/// Despawns entities when requested by the server.
fn despawn_entity_system(
    mut commands: Commands,
    manager: Res<EntityManager>,

    mut pks: EventReader<RemoveActor>,
) {
    for pk in pks.iter() {
        let entity = manager.entity_by_uid(pk.entity_unique_id);
        if entity.is_none() {
            error!(
                "Cannot remove unknown entity with unique id `{}`",
                pk.entity_unique_id.0
            );
            continue;
        }

        commands.entity(entity.unwrap()).despawn_recursive();
    }
}
