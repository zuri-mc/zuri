mod chunk_mesh;
pub mod component;
mod mesh;

use crate::client::NetworkSet;
use bevy::prelude::World as ECSWorld;
use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;
use zuri_net::proto::io::Reader;
use zuri_net::proto::packet::level_chunk::LevelChunk;
use zuri_net::proto::packet::network_chunk_publisher_update::NetworkChunkPublisherUpdate;
use zuri_net::proto::packet::start_game::StartGame;
use zuri_net::proto::packet::update_block::UpdateBlock;
use zuri_world::block;
use zuri_world::block::component::ComponentStorageType;
use zuri_world::block::{BlockBuilder, BlockMap, BlockMapBuilder, BlockType, PropertyValues};
use zuri_world::chunk::{Chunk, ChunkPos};
use zuri_world::pos::ChunkIndex;
use zuri_world::range::YRange;

/// Handles rendering and loading the chunks that make up the world.
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(
            BlockMapBuilder::vanilla()
                .with_component_type::<component::Geometry>(ComponentStorageType::Vector)
                .with_build_function(|block_map| {
                    let air_rid = block_map
                        .block(BlockBuilder::new(block::AIR_ID))
                        .unwrap()
                        .runtime_id()
                        .0;
                    // todo: load models & textures
                    let solid_block = component::Geometry::from_mesh(mesh::solid_block());
                    for rid in 0..block_map.runtime_ids() {
                        // Hard-code air for now.
                        if rid == air_rid {
                            continue;
                        }
                        block_map.set_component(rid, solid_block.clone())
                    }
                }),
        )
        .insert_resource(BlockTextures::default())
        .insert_resource(ChunkManager::default())
        // Startup systems
        .add_startup_system(textures_init_system)
        // Systems
        .add_system(build_block_map_system)
        .add_systems((
            chunk_unload_system.in_base_set(CoreSet::FixedUpdate),
            update_chunk_radius_system.in_base_set(NetworkSet::Process),
            chunk_update_system
                .in_base_set(CoreSet::PostUpdate)
                .run_if(world_is_loaded),
            block_update_system.in_base_set(CoreSet::PreUpdate),
        ))
        .add_systems((chunk_load_system.run_if(world_is_loaded),));
    }
}

/// Contains an arc pointer to the world's [BlockMap].
#[derive(Resource)]
pub struct World {
    /// A map of all registered blocks and their components.
    pub block_map: Arc<BlockMap>,
    /// How tall the current dimension is.
    pub y_range: YRange,
}

/// Keeps track of all chunks present in the world.
/// Contains a reference to the entity that contains a chunk's data given a certain chunk position.
#[derive(Resource, Default, Debug)]
pub struct ChunkManager {
    /// The center block position around which blocks will load. This is a position of a block, not
    /// a chunk itself.
    chunk_origin: IVec3,
    /// The radius around the chunk_origin where chunks are allowed to remain loaded in amount of
    /// blocks.
    chunk_radius: u32,
    chunks: HashMap<ChunkPos, Entity>,
}

#[allow(dead_code)]
impl ChunkManager {
    /// Returns the center of where chunks should remain loaded along with the amount of blocks
    /// around this center (block) position that should remain loaded.
    pub fn chunk_radius(&self) -> (IVec3, u32) {
        (self.chunk_origin, self.chunk_radius)
    }

    /// Changes where and how much chunks should remain loaded.
    pub fn set_chunk_radius(&mut self, origin: IVec3, radius: u32) {
        self.chunk_origin = origin;
        self.chunk_radius = radius;
    }

    /// Returns the chunk entity associated to the chunk position passed as an argument.
    /// If there is no loaded chunk at the position, None is returned.
    pub fn get(&self, chunk_pos: ChunkPos) -> Option<Entity> {
        self.chunks.get(&chunk_pos).copied()
    }

    /// Changes what chunk entity a chunk position refers to.
    /// If None is passed, the chunk is removed from the chunk manager (but not from the world).
    pub fn set(&mut self, chunk_pos: ChunkPos, val: Option<Entity>) -> Option<Entity> {
        match val {
            Some(inner) => self.chunks.insert(chunk_pos, inner),
            None => self.chunks.remove(&chunk_pos),
        }
    }

    /// Returns the chunk in which the provided world position is in (if it is loaded).
    /// The provided Y-value does not have an effect on the result.
    pub fn at_pos(&self, world_pos: Vec3) -> Option<Entity> {
        self.get(IVec2::new(
            world_pos.x.floor() as i32 >> 4,
            world_pos.z.floor() as i32 >> 4,
        ))
    }

    /// Returns the chunk in which the provided block position is in (if it is loaded).
    /// The provided Y-value does not have an effect on the result.
    pub fn at_block_pos(&self, world_pos: IVec3) -> Option<Entity> {
        self.get(IVec2::new(world_pos.x >> 4, world_pos.z >> 4))
    }

    /// Returns an iterator over all loaded chunks.
    pub fn iter(&self) -> impl Iterator<Item = (&ChunkPos, &Entity)> {
        self.chunks.iter()
    }

    /// Removes all known chunks from the chunk manager. Does NOT remove these entities from the
    /// world.
    pub fn clear(&mut self) {
        self.chunks.clear();
    }
}

/// Condition system that can be used to only run systems when there is a world loaded.
pub fn world_is_loaded(world: Option<Res<World>>) -> bool {
    world.is_some()
}

/// Builds the [BlockMap] when the StartGame packet is received.
fn build_block_map_system(world: &mut ECSWorld) {
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

    // Read custom blocks from the StartGame packet.
    for entry in &start_game.blocks {
        #[derive(Deserialize, Debug)]
        struct BlockProperties {
            #[serde(default)]
            properties: Vec<BlockProperty>,
        }

        #[derive(Deserialize, Debug)]
        struct BlockProperty {
            name: String,
            r#enum: BlockPropertyEnum,
        }

        #[derive(Deserialize, Debug)]
        #[serde(untagged)]
        enum BlockPropertyEnum {
            Strings(Vec<Box<str>>),
            Ints(Vec<i32>),
            Bool(Vec<bool>),
        }

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
                match property.r#enum {
                    BlockPropertyEnum::Strings(v) => PropertyValues::Strings(v.into()),
                    BlockPropertyEnum::Ints(v) => PropertyValues::Ints(v.into()),
                    BlockPropertyEnum::Bool(_) => PropertyValues::Bool,
                },
            );
        }

        builder.insert_block(block_type);
    }

    world.insert_resource(World {
        block_map: Arc::new(builder.build()),
        y_range: YRange::new(-64, 319),
    });
    world.resource_mut::<Events<StartGame>>().clear();
}

/// Updates the mesh of a chunk when it has been modified.
fn chunk_update_system(
    world: Res<World>,
    mut assets: ResMut<Assets<Mesh>>,
    mut query: Query<(&mut Handle<Mesh>, &Chunk), Changed<Chunk>>,
) {
    for (mesh, chunk) in &mut query {
        assets.set_untracked(
            mesh.id(),
            chunk_mesh::build_mesh(world.block_map.as_ref(), chunk),
        );
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
            query
                .get_mut(chunk_entity)
                .unwrap()
                .set(
                    <ChunkIndex as From<IVec3>>::from(pk.position.into()),
                    pk.new_block_runtime_id.0,
                )
                .unwrap();
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
    world: Res<World>,
) {
    if events.is_empty() {
        return;
    }

    for event in events.iter() {
        let mut reader = Reader::from_buf(event.raw_payload.clone(), 0);

        let chunk = Chunk::read(
            &mut reader,
            world.y_range,
            event.sub_chunk_count,
            world.block_map.clone(),
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
                    mesh: meshes.add(chunk_mesh::build_mesh(&world.block_map, &chunk)),
                    material: materials.add(StandardMaterial {
                        base_color_texture: Some(block_tex.dirt.clone().unwrap()),
                        base_color: Color::WHITE,
                        alpha_mode: AlphaMode::Opaque,
                        perceptual_roughness: 0.94,
                        ..default()
                    }),
                    transform: Transform::from_xyz(pos.x as f32, 0., pos.y as f32),
                    ..default()
                },
                chunk,
            ))
            .id();

        chunks.set(event.position, Some(entity));
    }
}

/// Unloads chunks that are too far from where the chunk origin is.
fn chunk_unload_system(mut commands: Commands, mut chunks: ResMut<ChunkManager>) {
    let (origin, radius) = chunks.chunk_radius();
    let origin = Vec2::new(origin.x as f32 + 0.5, origin.z as f32 + 0.5);
    let radius_squared = (radius * radius) as f32;

    chunks.chunks.retain(|pos, entity| {
        let chunk_center = Vec2::new((pos.x * 16) as f32 + 8., (pos.y * 16) as f32 + 8.);

        if (chunk_center - origin).length_squared() <= radius_squared {
            return true;
        }
        debug!("Unloading chunk {pos}");
        commands.entity(entity.clone()).despawn();
        false
    });
}

/// Updates the chunk radius when the server tells the client to.
fn update_chunk_radius_system(
    mut events: EventReader<NetworkChunkPublisherUpdate>,
    mut chunks: ResMut<ChunkManager>,
) {
    for event in events.iter() {
        chunks.set_chunk_radius(event.position.0, event.radius.0);
    }
}
