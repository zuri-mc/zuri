use bevy::prelude::*;
use bevy::reflect::erased_serde::__private::serde::Deserialize;
use bevy::render::mesh::PrimitiveTopology;
use zuri_net::proto::packet::start_game::StartGame;
use zuri_net::proto::packet::update_block::UpdateBlock;
use zuri_world::block::component::geometry::Geometry;
use zuri_world::block::{BlockBuilder, BlockMap, BlockMapBuilder, BlockType, PropertyValues};
use zuri_world::chunk::{Chunk, ChunkManager};
use zuri_world::pos::ChunkIndex;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BlockMapBuilder::vanilla())
            .add_system(build_block_map_system)
            .add_system_to_stage(CoreStage::PostUpdate, chunk_update_system)
            .add_system_to_stage(CoreStage::PreUpdate, block_update_system);
    }
}

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

/// Builds the [BlockMap] when the StartGame packet is received.
fn build_block_map_system(world: &mut World) {
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
