extern crate core;

use std::f32::consts::PI;

use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::window::{CursorGrabMode, PresentMode};
use bevy::{
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    prelude::*,
    render::{render_resource::WgpuFeatures, settings::WgpuSettings},
};

use dotenvy::dotenv;
use zuri_net::proto::io::Reader;
use zuri_net::proto::packet::level_chunk::LevelChunk;
use zuri_world::block::component::geometry::Geometry;
use zuri_world::block::{BlockMap, ToRuntimeId};

use zuri_world::chunk::{Chunk, ChunkManager};
use zuri_world::range::YRange;

use crate::client::ClientPlugin;
use crate::entity::Head;
use crate::input::InputPlugin;
use crate::player::{Local, LocalPlayerPlugin};
use crate::world::WorldPlugin;

mod client;
mod entity;
mod input;
mod player;
mod world;

#[tokio::main]
async fn main() {
    // Load environment variables from a `.env` file.
    dotenv().ok();
    App::new()
        .insert_resource(WgpuSettings {
            features: WgpuFeatures::POLYGON_MODE_LINE,
            ..default()
        })
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "Minecraft".into(),
                        present_mode: PresentMode::Immediate,
                        ..default()
                    },
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(WireframePlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_plugin(ClientPlugin)
        .add_plugin(InputPlugin)
        .add_plugin(LocalPlayerPlugin)
        .add_plugin(WorldPlugin)
        .insert_resource(BlockTextures::default())
        .insert_resource(ChunkManager::default())
        .add_startup_system(setup)
        .add_system(cursor_grab_system)
        .add_system(chunk_load_system)
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

#[derive(Resource, Default)]
pub struct BlockTextures {
    // only support one block for now
    dirt: Option<Handle<Image>>,
}

fn chunk_load_system(
    mut commands: Commands,
    mut events: EventReader<LevelChunk>,
    mut chunks: ResMut<ChunkManager>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut world_chunks: Query<&mut Chunk>,
    block_tex: Res<BlockTextures>,
    blocks: Option<Res<BlockMap>>,
) {
    if blocks.is_none() {
        return;
    }
    let blocks = blocks.unwrap();

    for event in events.iter() {
        let mut reader = Reader::from_buf(event.raw_payload.clone(), 0);
        // If the chunk already exists, so replace its contents.
        if let Some(entity) = chunks.get(event.position) {
            *world_chunks.get_mut(entity).unwrap() = Chunk::read(
                &mut reader,
                YRange::new(-64, 319),
                event.sub_chunk_count,
                blocks
                    .block_type("minecraft:air")
                    .unwrap()
                    .variants()
                    .next()
                    .unwrap()
                    .to_runtime_id(&blocks)
                    .into(), // todo: improve
            );
            continue;
        }

        let pos = event.position * 16;

        let s = Chunk::read(
            &mut reader,
            YRange::new(-64, 319),
            event.sub_chunk_count,
            blocks
                .block_type("minecraft:air")
                .unwrap()
                .variants()
                .next()
                .unwrap()
                .to_runtime_id(&blocks)
                .into(), // todo: improve
        );
        let entity = commands
            .spawn((
                PbrBundle {
                    mesh: meshes.add(s.build_mesh(blocks.components::<Geometry>())),
                    material: materials.add(StandardMaterial {
                        base_color_texture: Some(block_tex.dirt.clone().unwrap()),
                        base_color: Color::WHITE,
                        alpha_mode: AlphaMode::Opaque,
                        perceptual_roughness: 0.94,
                        ..default()
                    }),
                    transform: Transform::from_xyz(pos.x as f32, -32., pos.y as f32),
                    ..default()
                },
                s,
            ))
            .id();

        chunks.set(event.position, Some(entity));
    }
}

fn setup(
    mut commands: Commands,
    mut wireframe_config: ResMut<WireframeConfig>,
    mut block_tex: ResMut<BlockTextures>,
    asset_server: Res<AssetServer>,
) {
    wireframe_config.global = false;

    let texture_handle = asset_server.load("dirt.png");
    block_tex.dirt = Some(texture_handle);

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
    commands
        .spawn(TransformBundle {
            local: Transform::from_xyz(-5.0, 2.5, 5.0),
            ..default()
        })
        .insert((Head::default(), Local));
}
