use bevy::input::mouse::MouseMotion;
use bevy::pbr::wireframe::WireframeConfig;
use bevy::prelude::*;
use bevy::window::CursorGrabMode;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ClientInput::default())
            .add_system(mouse_input_system)
            .add_system(keyboard_input_system);
    }
}

#[derive(Resource, Default)]
pub struct ClientInput {
    pub movement: Vec2,
    pub rotation: Vec2,
    pub sprint: bool,
    pub jump: bool,
    pub sneak: bool,
}

fn keyboard_input_system(keyboard: Res<Input<KeyCode>>, mut input: ResMut<ClientInput>, mut wireframe_config: ResMut<WireframeConfig>,) {
    input.movement = Vec2::new(
        if keyboard.pressed(KeyCode::D) {
            1.
        } else if keyboard.pressed(KeyCode::A) {
            -1.
        } else {
            0.
        },
        if keyboard.pressed(KeyCode::S) {
            1.
        } else if keyboard.pressed(KeyCode::W) {
            -1.
        } else {
            0.
        },
    ).normalize_or_zero();
    input.jump = keyboard.pressed(KeyCode::Space);
    input.sprint = keyboard.pressed(KeyCode::LShift);
    input.sneak = keyboard.pressed(KeyCode::LControl);
    wireframe_config.global = keyboard.pressed(KeyCode::F);
}

fn mouse_input_system(
    windows: Res<Windows>,
    mut mouse_mot_event: EventReader<MouseMotion>,
    mut input: ResMut<ClientInput>,
) {
    input.rotation = Vec2::ZERO;

    let window = windows.primary();
    if window.cursor_grab_mode() != CursorGrabMode::Locked {
        return;
    }

    for e in mouse_mot_event.iter() {
        input.rotation += e.delta;
    }
    input.rotation /= window.height();
}
