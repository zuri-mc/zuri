use bytes::Bytes;
use zuri_nbt::Value;
use zuri_nbt::encoding::NetworkLittleEndian;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use zuri_net_derive::packet;
use crate::proto::ints::VarI32;

use crate::proto::io::{Reader, Writer};

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum SpawnBiomeType {
    Default,
    USerDefined,
}

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum SpawnType {
    Player,
    World,
}

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum SubChunkRequestMode {
    Legacy,
    Limitless,
    Limited,
}

#[derive(Debug, Clone, PartialEq, FromPrimitive, ToPrimitive)]
pub enum SubChunkResult {
    Success,
    ChunkNotFound,
    InvalidDimension,
    PlayerNotFound,
    IndexOutOfBounds,
    SuccessAllAir,
}

#[derive(Debug, Copy, Clone, FromPrimitive, ToPrimitive)]
pub enum Difficulty {
    Peaceful,
    Easy,
    Normal,
    Hard,
}

#[derive(Debug, Copy, Clone, FromPrimitive, ToPrimitive)]
pub enum Dimension {
    Overworld,
    Nether,
    End,
}

#[derive(Debug, Clone, PartialEq, FromPrimitive, ToPrimitive)]
pub enum HeightMapType {
    None,
    HasData,
    TooHigh,
    TooLow,
}

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum GameType {
    Survival,
    Creative,
    Adventure,
    SurvivalSpectator,
    CreativeSpectator,
    Default,
    Spectator,
}

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum Generator {
    Legacy,
    Overworld,
    Flat,
    Nether,
    End,
    Void,
}

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum PermissionLevel {
    Visitor,
    Member,
    Operator,
    Custom,
}

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum EntityLinkType {
    Remove,
    Rider,
    Passenger,
}

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum UpdateBlockTransition {
    BlockToEntity,
    EntityToBlock,
}

#[derive(Debug, Clone)]
pub struct BlockEntry {
    pub name: String,
    pub properties: Value,
}

impl BlockEntry {
    pub fn write(&self, writer: &mut Writer) {
        writer.string(self.name.as_str());
        writer.nbt(&self.properties, NetworkLittleEndian);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            name: reader.string(),
            properties: reader.nbt(NetworkLittleEndian),
        }
    }
}

#[packet]
#[derive(Debug, Clone)]
pub struct GenerationFeature {
    name: String,
    json: Bytes,
}

#[packet]
#[derive(Debug, Clone)]
pub struct DimensionDefinition {
    name: String,
    range: [VarI32; 2],
    generator: VarI32,
}

#[derive(Debug, Clone)]
pub struct EntityLink {
    pub ridden_entity_unique_id: i64,
    pub rider_entity_unique_id: i64,
    pub link_type: EntityLinkType,
    pub immediate: bool,
    pub rider_initiated: bool,
}

impl EntityLink {
    pub fn write(&self, writer: &mut Writer) {
        writer.var_i64(self.ridden_entity_unique_id);
        writer.var_i64(self.rider_entity_unique_id);
        writer.u8(self.link_type.to_u8().unwrap());
        writer.bool(self.immediate);
        writer.bool(self.rider_initiated);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            ridden_entity_unique_id: reader.var_i64(),
            rider_entity_unique_id: reader.var_i64(),
            link_type: EntityLinkType::from_u8(reader.u8()).unwrap(),
            immediate: reader.bool(),
            rider_initiated: reader.bool(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SubChunkEntry {
    pub offset: SubChunkOffset,
    pub result: SubChunkResult,
    pub raw_payload: Bytes,
    pub height_map_type: HeightMapType,
    pub height_map_data: [i8; 256],
    pub blob_hash: u64,
}

impl SubChunkEntry {
    pub fn write(&self, writer: &mut Writer, cache_enabled: bool) {
        self.offset.write(writer);
        writer.u8(self.result.to_u8().unwrap());
        if self.result != SubChunkResult::SuccessAllAir || cache_enabled {
            writer.byte_slice(&self.raw_payload);
        }
        writer.u8(self.height_map_type.to_u8().unwrap());
        if self.height_map_type == HeightMapType::HasData {
            for data in self.height_map_data {
                writer.i8(data);
            }
        }
        if !cache_enabled {
            writer.u64(self.blob_hash);
        }
    }

    pub fn read(reader: &mut Reader, cache_enabled: bool) -> Self {
        let mut entry = Self {
            offset: SubChunkOffset::read(reader),
            result: SubChunkResult::from_u8(reader.u8()).unwrap(),
            raw_payload: Bytes::default(),
            height_map_type: HeightMapType::None,
            height_map_data: [0; 256],
            blob_hash: 0,
        };
        if entry.result != SubChunkResult::SuccessAllAir || cache_enabled {
            entry.raw_payload = reader.byte_slice();
        }
        entry.height_map_type = HeightMapType::from_u8(reader.u8()).unwrap();
        if entry.height_map_type == HeightMapType::HasData {
            for i in 0..256 {
                entry.height_map_data[i] = reader.i8();
            }
        }
        if !cache_enabled {
            entry.blob_hash = reader.u64();
        }

        entry
    }
}

#[derive(Debug, Clone)]
pub struct SubChunkOffset {
    pub x: i8,
    pub y: i8,
    pub z: i8,
}

impl SubChunkOffset {
    pub fn write(&self, writer: &mut Writer) {
        writer.i8(self.x);
        writer.i8(self.y);
        writer.i8(self.z);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            x: reader.i8(),
            y: reader.i8(),
            z: reader.i8(),
        }
    }
}

#[packet]
#[derive(Debug, Clone)]
pub struct CacheBlob {
    pub hash: u64,
    pub payload: Bytes,
}

#[packet]
#[derive(Debug, Clone)]
pub struct ExperimentData {
    pub name: String,
    pub enabled: bool,
}
