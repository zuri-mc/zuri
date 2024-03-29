extern crate core;

use std::f32::consts::PI;

use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::window::{CursorGrabMode, PresentMode};
use bevy::{
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    prelude::*,
};

use dotenvy::dotenv;

use crate::client::ClientPlugin;
use crate::entity::{EntityPlugin, Head};
use crate::input::InputPlugin;
use crate::player::{Local, LocalPlayerPlugin};
use crate::world::WorldPlugin;

pub mod client;
pub mod entity;
mod input;
pub mod player;
mod world;

#[tokio::main]
async fn main() {
    // Load environment variables from a `.env` file.
    dotenv().ok();
    App::new()
        .insert_resource(FixedTime::new_from_secs(1. / 20.))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Zuri".into(),
                        present_mode: PresentMode::Immediate,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(WireframePlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_plugin(EntityPlugin)
        .add_plugin(ClientPlugin)
        .add_plugin(InputPlugin)
        .add_plugin(LocalPlayerPlugin)
        .add_plugin(WorldPlugin)
        .add_startup_system(setup)
        .add_system(cursor_grab_system)
        .run();
}

fn cursor_grab_system(
    btn: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,

    mut windows: Query<&mut Window>,
) {
    if let Some(mut window) = windows.iter_mut().next() {
        if btn.just_pressed(MouseButton::Left) {
            window.cursor.grab_mode = CursorGrabMode::Locked;
            window.cursor.visible = false;
        }

        if key.just_pressed(KeyCode::Escape) {
            window.cursor.grab_mode = CursorGrabMode::None;
            window.cursor.visible = true;
        }
    }
}

fn setup(mut commands: Commands, mut wireframe_config: ResMut<WireframeConfig>) {
    wireframe_config.global = false;

    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.3,
    });
    // sunlight
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 3600.,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        camera_3d: Camera3d {
            clear_color: ClearColorConfig::Custom(Color::rgb(0.5, 0.6, 0.8)),
            ..default()
        },
        transform: Transform::from_xyz(-5.0, 2.5, 5.0),
        ..default()
    });
    commands
        .spawn(TransformBundle {
            local: Transform::from_xyz(-5.0, 2.5, 5.0),
            ..default()
        })
        .insert((Head::default(), Local));
}
