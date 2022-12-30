use bevy::prelude::*;
use bevy::time::FixedTimestep;

use zuri_net::client::login::LoginData;
use zuri_net::proto::packet::move_actor_absolute::MoveActorAbsolute;
use zuri_net::proto::packet::move_player::{MoveMode, MovePlayer, TeleportCause};
use zuri_net::proto::packet::Packet;
use zuri_net::proto::packet::player_auth_input::{PlayerAuthInput, PlayMode};
use zuri_net::proto::types::player::{InputMode, InteractionModel, PlayerMovementMode};

use crate::client::{LoginDataResource, LoginEvent};
use crate::entity::{Head, RuntimeId};
use crate::input::ClientInput;

pub struct LocalPlayerPlugin;

impl Plugin for LocalPlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(spawn_player)
            .add_system(move_system)
            .add_system(camera_sync_system)
            .add_system(server_pos_sync)
            .add_system_set(
                SystemSet::new()
                    // This prints out "goodbye world" twice every second
                    .with_run_criteria(FixedTimestep::step(1. / 20.))
                    .with_system(client_pos_sync)
            );
    }
}

/// Flag component to indicate the locally controlled player.
#[derive(Component)]
pub struct Local; // todo: use SparseSet

fn spawn_player(mut commands: Commands, mut event: EventReader<LoginEvent>) {
    if event.is_empty() {
        return;
    }
    let data = event.iter().next().unwrap();
    commands.spawn(TransformBundle {
        local: Transform::from_translation(data.0.position)
            .with_rotation(Quat::from_rotation_y(data.0.yaw)),
        ..default()
    }).insert((
        Head::new(data.0.pitch, 0.),
        Local,
        RuntimeId(data.0.player_runtime_id),
    ));
}

fn move_system(time: Res<Time>, input: Res<ClientInput>, mut query: Query<(&mut Transform, &mut Head), With<Local>>) {
    if let Ok((mut tr, mut head)) = query.get_single_mut() {
        tr.rotation *= Quat::from_rotation_y(-input.rotation.x);
        head.rot *= Quat::from_rotation_x(-input.rotation.y);

        let rotation = tr.rotation;
        let speed = 10. * if input.sprint { 2. } else { 1. };
        tr.translation += rotation * Vec3::new(
            input.movement.x,
            if input.jump { 0.8 } else if input.sneak { -0.8 } else { 0. },
            input.movement.y,
        ) * time.delta_seconds() * speed;
    }
}

fn camera_sync_system(player_query: Query<(&Transform, &Head), (With<Local>, Without<Camera3d>)>, mut cam_query: Query<&mut Transform, With<Camera3d>>) {
    if let Ok((tr, head)) = player_query.get_single() {
        let mut cam_transform = cam_query.single_mut();

        cam_transform.translation = tr.translation + head.eye_height;
        cam_transform.rotation = tr.rotation * head.rot;
    }
}

fn client_pos_sync(
    query: Query<(&Transform, &Head, &RuntimeId), (With<Local>, Or<(Changed<Transform>, Changed<Head>)>)>,
    mut sender: EventWriter<Packet>,
    data: Option<Res<LoginDataResource>>,
    input: Res<ClientInput>,
) {
    if let Ok((tra, head, rid)) = query.get_single() {
        let pitch = head.rot.to_euler(EulerRot::XYZ).0.to_degrees();
        let yaw = tra.rotation.to_euler(EulerRot::XYZ).1.to_degrees();
        match data.unwrap().0.player_movement_settings.movement_type {
            PlayerMovementMode::Client => sender.send(MovePlayer {
                entity_runtime_id: rid.0,
                position: tra.translation,
                pitch,
                yaw,
                head_yaw: yaw,
                mode: MoveMode::Normal,
                on_ground: true,
                ridden_entity_runtime_id: 0,
                teleport_cause: TeleportCause::None,
                teleport_source_entity_type: 0,
                tick: 0,
            }.into()),
            PlayerMovementMode::Server => sender.send(PlayerAuthInput {
                pitch,
                yaw,
                head_yaw: yaw,
                position: tra.translation,
                move_vector: input.movement,
                input_data: 0,
                input_mode: InputMode::Mouse,
                play_mode: PlayMode::Normal,
                interaction_model: InteractionModel::Crosshair,
                gaze_direction: Default::default(),
                tick: 0,
                delta: Default::default(),
                item_interaction_data: Default::default(),
                item_stack_request: Default::default(),
                block_actions: vec![],
            }.into()),
            PlayerMovementMode::ServerWithRewind => todo!(),
        }
    }
}

fn server_pos_sync(
    mut query: Query<(&mut Transform, &mut Head, &RuntimeId), With<Local>>,
    mut pks: EventReader<MoveActorAbsolute>,
) {
    for pk in pks.iter() {
        for (mut tr, mut head, rid) in &mut query {
            if pk.entity_runtime_id != rid.0 {
                continue;
            }
            tr.translation = pk.position;
            // todo: rot
            tr.rotation = Quat::from_rotation_y(pk.rotation.y);
            head.rot = Quat::from_rotation_x(pk.rotation.x)
        }
    }
}
