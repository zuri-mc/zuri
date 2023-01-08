extern crate core;

use std::f32::consts::PI;

use bevy::{
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    prelude::*,
    render::{render_resource::WgpuFeatures, settings::WgpuSettings},
};
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::pbr::wireframe::Wireframe;
use bevy::window::{CursorGrabMode, PresentMode};
use zuri_model::geometry::{Geometry, GeometryList};
use zuri_model::model::{Model, ModelOptions};

use dotenvy::dotenv;
use zuri_net::proto::packet::level_chunk::LevelChunk;
use zuri_net::proto::io::Reader;

use zuri_world::chunk::Chunk;
use zuri_world::range::YRange;
use zuri_world::WorldPlugin;

use crate::client::ClientPlugin;
use crate::entity::Head;
use crate::input::InputPlugin;
use crate::player::{Local, LocalPlayerPlugin};

mod entity;
mod player;
mod input;
mod client;

#[tokio::main]
async fn main() {
    // Load environment variables from a `.env` file.
    dotenv().ok();
    App::new()
        .insert_resource(WgpuSettings {
            features: WgpuFeatures::POLYGON_MODE_LINE,
            ..default()
        })
        .init_resource::<BlockHandles>()
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

        //.add_plugin(ClientPlugin)
        .add_plugin(InputPlugin)
        .add_plugin(LocalPlayerPlugin)
        .add_plugin(WorldPlugin)

        .insert_resource(BlockTextures::default())
        .add_state(AppState::Setup)
        .add_system_set(SystemSet::on_enter(AppState::Setup).with_system(load_textures))
        .add_system_set(SystemSet::on_update(AppState::Setup).with_system(check_textures))
        .add_system_set(SystemSet::on_enter(AppState::Finished).with_system(setup))
        .add_system(cursor_grab_system)
        //.add_system(chunk_load_system)
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    block_tex: Res<BlockTextures>,
) {
    for event in events.iter() {
        let pos = event.position * 16;

        let mut reader = Reader::from_buf(event.raw_payload.clone(), 0);
        let s = Chunk::read(&mut reader, YRange::new(-64, 319), event.sub_chunk_count, 10462);
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(s.build_mesh()),
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
        ));
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    Setup,
    Finished,
}

#[derive(Resource, Default)]
struct BlockHandles {
    handles: Vec<HandleUntyped>,
}

fn load_textures(mut block_handles: ResMut<BlockHandles>, asset_server: Res<AssetServer>) {
    block_handles.handles = asset_server.load_folder("textures/blocks").unwrap();
}

fn check_textures(
    mut state: ResMut<State<AppState>>,
    block_handles: ResMut<BlockHandles>,
    asset_server: Res<AssetServer>,
) {
    if let LoadState::Loaded =
        asset_server.get_group_load_state(block_handles.handles.iter().map(|handle| handle.id))
    {
        state.set(AppState::Finished).unwrap();
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut block_textures: ResMut<Assets<Image>>,
    block_handles: Res<BlockHandles>,
    asset_server: Res<AssetServer>,
) {
    let mut builder = TextureAtlasBuilder::default();
    for handle in &block_handles.handles {
        let handle = handle.typed_weak();
        let Some(texture) = block_textures.get(&handle) else {
            warn!("{:?} did not resolve to an `Image` asset.", asset_server.get_handle_path(handle));
            continue;
        };

        builder.add_texture(handle, texture);
    }
    let texture_atlas = builder.finish(&mut block_textures).unwrap();
    let texture_atlas_texture = texture_atlas.texture.clone();

    let dirt_handle = asset_server.get_handle("textures/blocks/grass_side.tga");
    let dirt_index = texture_atlas.get_texture_index(&dirt_handle).unwrap();

    let mut geometries = GeometryList::new(include_str!("../assets/models/blocks/cube.geo.json"));
    geometries.into_iter().for_each(|geometry| {
        let model = Model::new(geometry, ModelOptions {
            texture_size: Some(texture_atlas.size),
            texture_offset: Some(texture_atlas.textures[dirt_index].clone().min / texture_atlas.size),
        });
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(model.build_mesh()),
                material: materials.add(StandardMaterial {
                    base_color_texture: Some(texture_atlas_texture.clone()),
                    base_color: Color::WHITE,
                    alpha_mode: AlphaMode::Mask(1.),
                    perceptual_roughness: 0.94,
                    ..default()
                }),
                transform: Transform::from_xyz(0., 0., 0.),
                ..default()
            },
        ));
    });

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