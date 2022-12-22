extern crate core;

use bevy::{
    pbr::wireframe::{Wireframe, WireframeConfig, WireframePlugin},
    prelude::*,
    render::{render_resource::WgpuFeatures, settings::WgpuSettings},
};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::window::{CursorGrabMode, PresentMode};
use noise::{NoiseFn, Simplex};

use crate::chunk::SubChunk;
use crate::entity::Head;
use crate::input::InputPlugin;
use crate::player::{Local, LocalPlayerPlugin};

mod entity;
mod player;
mod input;
mod io;
mod chunk;
mod protocol;

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

fn cursor_grab_system(
    mut windows: ResMut<Windows>,
    btn: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    let window = windows.get_primary_mut().unwrap();

    if btn.just_pressed(MouseButton::Left) {
        window.set_cursor_grab_mode(CursorGrabMode::Locked);
        window.set_cursor_visibility(false);
    }

    if key.just_pressed(KeyCode::Escape) {
        window.set_cursor_grab_mode(CursorGrabMode::None);
        window.set_cursor_visibility(true);
    }
}

fn setup(
    mut commands: Commands,
    mut wireframe_config: ResMut<WireframeConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    wireframe_config.global = false;
    // cubes
    let mut cube_count = 0;
    let noise = Simplex::new(1);
    for chunk_x in 0..4 {
        for chunk_z in 0..4 {
            let mut s = SubChunk::default();
            for x in 0..16 {
                let world_x = chunk_x * 16 + x;

                for z in 0..16 {
                    let world_z = chunk_z * 16 + z;

                    let max = (noise.get([world_x as f64 / 50., world_z as f64 / 50.]) * 15.) as i32;
                    for y in 0..max + 10 {
                        cube_count += 1;
                        s.set(x, y as u8, z, true);
                    }
                }
            }

            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(s.build_mesh()),
                    material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                    transform: Transform::from_xyz(chunk_x as f32 * 16., 0., chunk_z as f32 * 16.),
                    ..default()
                },
                Wireframe,
            ));
        }
    }
    println!("Placed {} cubes", cube_count);

    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-5.0, 2.5, 5.0),
        ..default()
    });
    commands.spawn(TransformBundle {
        local: Transform::from_xyz(-5.0, 2.5, 5.0),
        ..default()
    }).insert((Head::default(), Local));
}