extern crate core;

use std::f32::consts::PI;

use bevy::{
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    prelude::*,
    render::{render_resource::WgpuFeatures, settings::WgpuSettings},
};
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::window::{CursorGrabMode, PresentMode};
use bytes::{Buf, Bytes};
use noise::{NoiseFn, SuperSimplex};
use zuri_nbt::encoding::NetworkLittleEndian;
use zuri_nbt::Value;

use zuri_net::client::plugin::ClientPlugin;
use zuri_proto::io::Reader;
use zuri_proto::packet::level_chunk::LevelChunk;
use zuri_world::chunk::Chunk;
use zuri_world::pos::ChunkPos;
use zuri_world::range::YRange;
use zuri_world::subchunk::SubChunk;
use zuri_world::WorldPlugin;

use crate::entity::Head;
use crate::input::InputPlugin;
use crate::player::{Local, LocalPlayerPlugin};

mod entity;
mod player;
mod input;

#[tokio::main]
async fn main() {
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

        .add_plugin(ClientPlugin)
        .add_plugin(InputPlugin)
        .add_plugin(LocalPlayerPlugin)
        .add_plugin(WorldPlugin)

        .insert_resource(BlockTextures::default())
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    block_tex: Res<BlockTextures>
) {
    for event in events.iter() {
        let pos = (event.position * 16);

        let mut s = decode_chunk(event);
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

fn decode_chunk(pk: &LevelChunk) -> Chunk {
    let mut reader = Reader::from_buf(pk.raw_payload.clone(), 0);
    let mut sub_chunks: Vec<Option<SubChunk>> = Vec::new();
    for _ in 0..24 {
        sub_chunks.push(None);
    }

    for mut sub_chunk_num in 0..pk.sub_chunk_count {
        let ver = reader.u8();
        assert!(ver == 8 || ver == 9);
        let layer_count = reader.u8();
        if ver == 9 {
            let u_index = reader.u8();
            // todo
        }

        for current_layer in 0..layer_count {
            let original_block_size = reader.u8();
            let block_size = original_block_size >> 1;

            let mut total_needed: i32 = 0;
            if block_size != 0 {
                total_needed = 4096 / (32 / block_size as i32);
            }
            if block_size == 3 || block_size == 5 || block_size == 6 {
                total_needed += 1;
            }

            let mut u32s = Vec::<u32>::with_capacity(total_needed as usize);
            for _ in 0..total_needed {
                u32s.push(
                    reader.u8() as u32
                        | (reader.u8() as u32) << 8
                        | (reader.u8() as u32) << 16
                        | (reader.u8() as u32) << 24
                );
            }


            let mut palette = Vec::new();

            let mut palette_count: i32 = 1;
            if block_size != 0 {
                palette_count = reader.var_i32();
            }
            if original_block_size&1 != 1 {
                for _ in 0..palette_count {
                    let mut buf = reader.into();
                    let nbt = Value::read(&mut buf, &mut NetworkLittleEndian).unwrap();
                    reader = Reader::from_buf(buf, 0);
                    if let Value::Compound(map) = nbt {
                        if let Value::String(s) = &map["name"] {
                            println!("{s}");
                            if s == "minecraft:air" {
                                palette.push(0);
                            } else {
                                palette.push(1);
                            }
                        }
                    } else {
                        todo!();
                    }
                }
            } else {
                for _ in 0..palette_count {
                    palette.push(reader.var_i32() as u32);
                }
            }
            let mut sub = SubChunk::default();

            let bits_per_index = (u32s.len() / 32 / 4) as u16;
            let index_mask = (1u32 << bits_per_index) - 1;
            let mut filled_bits_per_index = 0u16;
            if bits_per_index != 0 {
                filled_bits_per_index = 32 / bits_per_index * bits_per_index;
            }

            let mut first = None;
            for x in 0..16 {
                for y in 0..16 {
                    for z in 0..16 {
                        let palette_index = if bits_per_index == 0 {
                            0
                        } else {
                            let offset = (((x as u16) << 8)
                                | ((z as u16) << 4) | (y as u16)) * bits_per_index;
                            let u32_offset = offset / filled_bits_per_index;
                            let bit_offset = offset % filled_bits_per_index;
                            ((u32s[u32_offset as usize] >> bit_offset) as u32) & index_mask
                        };
                        let id = palette[palette_index as usize];
                        if let None = first {
                            first = Some(id);
                        }
                        sub.set(x, y, z, id != 10462); // 8333, 12466
                    }
                }
            }
            sub_chunks[sub_chunk_num as usize] = Some(sub);
        }
    }
    Chunk::from_subchunks(-64, sub_chunks)
}

fn setup(
    mut commands: Commands,
    mut wireframe_config: ResMut<WireframeConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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
    commands.spawn(TransformBundle {
        local: Transform::from_xyz(-5.0, 2.5, 5.0),
        ..default()
    }).insert((Head::default(), Local));
}