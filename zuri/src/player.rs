use bevy::prelude::*;

use crate::entity::Head;
use crate::input::ClientInput;

pub struct LocalPlayerPlugin;

impl Plugin for LocalPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(move_system);
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
        tr.translation += rotation * Vec3::new(input.movement.x * time.delta_seconds(), 0., input.movement.y * time.delta_seconds()) * speed;
    }
}
