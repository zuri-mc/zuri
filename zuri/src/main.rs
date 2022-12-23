extern crate core;

use std::f32::consts::PI;
use bevy::{
    pbr::wireframe::{Wireframe, WireframeConfig, WireframePlugin},
    prelude::*,
    render::{render_resource::WgpuFeatures, settings::WgpuSettings},
};
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::shape::Cube;
use bevy::render::render_resource::Texture;
use bevy::window::{CursorGrabMode, PresentMode};
use noise::{NoiseFn, Simplex};

use zuri_world::chunk::Chunk;
use zuri_world::pos::ChunkPos;
use zuri_world::range::YRange;
use zuri_world::WorldPlugin;

use crate::entity::Head;
use crate::input::InputPlugin;
use crate::player::{Local, LocalPlayerPlugin};

mod entity;
mod player;
mod input;

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
        }).set(ImagePlugin::default_nearest()))
        .add_plugin(WireframePlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin)

        .add_plugin(InputPlugin)
        .add_plugin(LocalPlayerPlugin)
        .add_plugin(WorldPlugin)

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
    asset_server: Res<AssetServer>,
) {
    wireframe_config.global = false;

    let texture_handle = asset_server.load("dirt.png");

    // cubes
    let mut cube_count = 0;
    let noise = Simplex::new(1);
    for chunk_x in 0..16 {
        for chunk_z in 0..16 {
            let mut s = Chunk::empty(YRange::new(0, 63));
            for x in 0..16 {
                let world_x = chunk_x * 16 + x;

                for z in 0..16 {
                    let world_z = chunk_z * 16 + z;

                    let max = (noise.get([world_x as f64 / 50., world_z as f64 / 50.]) * 15.) as i32;
                    for y in 0..max + 50 {
                        cube_count += 1;
                        s.set(ChunkPos::new(x, y as i16, z), true);
                    }
                }
            }

            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(s.build_mesh()),
                    material: materials.add(StandardMaterial {
                        base_color_texture: Some(texture_handle.clone()),
                        base_color: Color::WHITE,
                        alpha_mode: AlphaMode::Opaque,
                        //reflectance: 0.01,
                        perceptual_roughness: 0.94,
                        //unlit: true,
                        ..default()
                    }),
                    transform: Transform::from_xyz(chunk_x as f32 * 16., -32., chunk_z as f32 * 16.),
                    ..default()
                },
                //Wireframe,
                s,
            ));
        }
    }
    println!("Placed {} cubes", cube_count);

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
    const HALF_SIZE: f32 = 10.0;
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            // Configure the projection to better fit the scene
            shadow_projection: OrthographicProjection {
                left: -HALF_SIZE,
                right: HALF_SIZE,
                bottom: -HALF_SIZE,
                top: HALF_SIZE,
                near: -10.0 * HALF_SIZE,
                far: 10.0 * HALF_SIZE,
                ..default()
            },
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
    commands.spawn(TransformBundle {
        local: Transform::from_xyz(-5.0, 2.5, 5.0),
        ..default()
    }).insert((Head::default(), Local));
}