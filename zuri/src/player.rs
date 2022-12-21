use bevy::prelude::*;

use crate::entity::Head;
use crate::input::ClientInput;

pub struct LocalPlayerPlugin;

impl Plugin for LocalPlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(move_system)
            .add_system(camera_sync_system);
    }
}

/// Flag component to indicate the locally controlled player.
#[derive(Component)]
pub struct Local;

fn move_system(time: Res<Time>, input: Res<ClientInput>, mut query: Query<(&mut Transform, &mut Head), With<Local>>) {
    if let Ok((mut tr, mut head)) = query.get_single_mut() {
        tr.rotation *= Quat::from_rotation_y(-input.rotation.x);
        head.rot *= Quat::from_rotation_x(-input.rotation.y);

        let rotation = tr.rotation;
        let speed = 4. * if input.sprint { 2. } else { 1. };
        tr.translation += rotation * Vec3::new(
            input.movement.x,
            if input.jump { 0.8 } else if input.sneak { -0.8 } else { 0. },
            input.movement.y,
        ) * time.delta_seconds() * speed;
    }
}

fn camera_sync_system(player_query: Query<(&Transform, &Head), (With<Local>, Without<Camera3d>)>, mut cam_query: Query<(&mut Transform), With<Camera3d>>) {
    if let Ok((tr, head)) = player_query.get_single() {
        let mut cam_transform = cam_query.single_mut();

        cam_transform.translation = tr.translation + head.eye_height;
        cam_transform.rotation = tr.rotation * head.rot;
    }
}
