use bevy::prelude::*;
use bevy::render::mesh::PrimitiveTopology;
use serde::Deserialize;
use zuri_net::proto::io::Reader;
use zuri_net::proto::packet::level_chunk::LevelChunk;
use zuri_net::proto::packet::start_game::StartGame;
use zuri_net::proto::packet::update_block::UpdateBlock;
use zuri_world::block::component::geometry::Geometry;
use zuri_world::block::{
    BlockBuilder, BlockMap, BlockMapBuilder, BlockType, PropertyValues, ToRuntimeId,
};
use zuri_world::chunk::{Chunk, ChunkManager};
use zuri_world::pos::ChunkIndex;
use zuri_world::range::YRange;

/// Handles rendering and loading the chunks that make up the world.
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BlockMapBuilder::vanilla())
            .insert_resource(BlockTextures::default())
            .insert_resource(ChunkManager::default())
            // Startup systems
            .add_startup_system(textures_init_system)
            // Systems
            .add_system(build_block_map_system)
            .add_system(chunk_load_system)
            .add_system_to_stage(CoreStage::PostUpdate, chunk_update_system)
            .add_system_to_stage(CoreStage::PreUpdate, block_update_system);
    }
}

/// Builds the [BlockMap] when the StartGame packet is received.
fn build_block_map_system(world: &mut World) {
    #[derive(Deserialize, Debug)]
    struct BlockProperties {
        #[serde(default)]
        properties: Vec<BlockProperty>,
    }

    #[derive(Deserialize, Debug)]
    struct BlockProperty {
        name: String,
        #[serde(rename = "enum")]
        values: BlockPropertyEnum,
    }

    #[derive(Deserialize, Debug)]
    #[serde(untagged)]
    enum BlockPropertyEnum {
        Strings(Vec<Box<str>>),
        Ints(Vec<i32>),
        Bool(Vec<bool>),
    }

    let start_game_event = world.resource::<Events<StartGame>>();
    if start_game_event.is_empty() {
        return;
    }
    let mut event_reader = start_game_event.get_reader();

    let mut builder = world
        .remove_resource::<BlockMapBuilder>()
        .expect("BlockMapBuilder is missing on StartGame");

    let start_game = event_reader
        .iter(world.resource::<Events<StartGame>>())
        .next()
        .unwrap();

    for entry in &start_game.blocks {
        let mut block_type = BlockType::new(entry.name.as_str());

        let properties = zuri_nbt::serde::deserialize::<BlockProperties>(&entry.properties);
        if let Err(err) = properties {
            error!(
                "Could not read properties for custom block `{}`: {}",
                entry.name, err
            );
            continue;
        }
        let properties = properties.unwrap();

        for property in properties.properties {
            block_type.insert_property(
                property.name,
                match property.values {
                    BlockPropertyEnum::Strings(v) => PropertyValues::Strings(v.into()),
                    BlockPropertyEnum::Ints(v) => PropertyValues::Ints(v.into()),
                    BlockPropertyEnum::Bool(_) => PropertyValues::Bool,
                },
            );
        }

        builder.insert_block(block_type);
    }

    let mut block_map = builder.build();
    // Hard-code air for now.
    block_map.set_component(
        BlockBuilder::new("minecraft:air"),
        Geometry {
            mesh: Mesh::new(PrimitiveTopology::TriangleList),
        },
    );

    world.insert_resource(block_map);
}

/// Updates the mesh of a chunk when it has been modified.
fn chunk_update_system(
    mut assets: ResMut<Assets<Mesh>>,
    mut query: Query<(&mut Handle<Mesh>, &Chunk), Changed<Chunk>>,
    blocks: Option<Res<BlockMap>>,
) {
    if blocks.is_none() {
        return;
    }
    let blocks = blocks.unwrap();

    for (mesh, chunk) in &mut query {
        assets.set_untracked(mesh.id(), chunk.build_mesh(blocks.components::<Geometry>()));
    }
}

/// Updates a block in the world when the server sends a block update.
fn block_update_system(
    mut pks: EventReader<UpdateBlock>,
    chunks: Res<ChunkManager>,
    mut query: Query<&mut Chunk>,
) {
    for pk in pks.iter() {
        // Multi-layer chunks are not yet supported.
        if pk.layer.0 != 0 {
            continue;
        }
        if let Some(chunk_entity) = chunks.at_block_pos(pk.position.into()) {
            query.get_mut(chunk_entity).unwrap().set(
                <ChunkIndex as From<IVec3>>::from(pk.position.into()),
                pk.new_block_runtime_id.into(),
            );
        }
    }
}

/// Loads textures. For now, this is only dirt.
fn textures_init_system(mut block_tex: ResMut<BlockTextures>, asset_server: Res<AssetServer>) {
    let texture_handle = asset_server.load("dirt.png");
    block_tex.dirt = Some(texture_handle);
}

#[derive(Resource, Default)]
pub struct BlockTextures {
    // only support one block for now
    dirt: Option<Handle<Image>>,
}

/// Decodes and spawns chunks sent by the server
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

    if events.is_empty() {
        return;
    }

    let air_rid = blocks
        .block_type("minecraft:air")
        .unwrap()
        .variants()
        .next()
        .unwrap()
        .to_runtime_id(&blocks);

    for event in events.iter() {
        let mut reader = Reader::from_buf(event.raw_payload.clone(), 0);

        let chunk = Chunk::read(
            &mut reader,
            YRange::new(-64, 319),
            event.sub_chunk_count,
            air_rid.into(),
        );

        // If the chunk already exists, so replace its contents.
        if let Some(entity) = chunks.get(event.position) {
            *world_chunks.get_mut(entity).unwrap() = chunk;
            continue;
        }

        let pos = event.position * 16;

        let entity = commands
            .spawn((
                PbrBundle {
                    mesh: meshes.add(chunk.build_mesh(blocks.components::<Geometry>())),
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
                chunk,
            ))
            .id();

        chunks.set(event.position, Some(entity));
    }
}
