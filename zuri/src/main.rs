use bevy::{
    pbr::wireframe::{Wireframe, WireframeConfig, WireframePlugin},
    prelude::*,
    render::{render_resource::WgpuFeatures, settings::WgpuSettings},
};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::window::{CursorGrabMode, PresentMode};
use noise::{NoiseFn, Simplex};

use crate::entity::Head;
use crate::input::InputPlugin;
use crate::player::{Local, LocalPlayerPlugin};

mod entity;
mod player;
mod input;
mod io;

fn main() {
    App::new()
        .insert_resource(WgpuSettings {
            features: WgpuFeatures::POLYGON_MODE_LINE,
            ..default()
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Minecraft".into(),
                present_mode: PresentMode::Immediate,
                ..default()
            },
            ..default()
        }))
        .add_plugin(WireframePlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin)

        .add_plugin(InputPlugin)
        .add_plugin(LocalPlayerPlugin)

        .add_startup_system(setup)
        .add_system(cursor_grab_system)
        .run();
}

fn setup(
    mut commands: Commands,
    mut wireframe_config: ResMut<WireframeConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    wireframe_config.global = false;
    // cubes
    let noise = Simplex::new(1);
    let size = 0.2;
    for x in 0..32 {
        for z in 0..32 {
            let max = (noise.get([x as f64 / 50., z as f64 / 50.]) * 15.) as i32;
            for y in -3..max {
                commands.spawn((
                    PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube { size })),
                        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                        transform: Transform::from_xyz(x as f32 * size - 3.2, y as f32 * size, z as f32 * size - 3.2),
                        ..default()
                    },
                    Wireframe,
                ));
            }
        }
    }
    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-5.0, 2.5, 5.0),
        ..default()
    }).insert((Head::default(), Local));
}