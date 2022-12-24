use bytes::Bytes;
use num_derive::{FromPrimitive, ToPrimitive};
use zuri_nbt::encoding::NetworkLittleEndian;
use zuri_nbt::Value;
use crate::io::{Reader, Writer};

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum SpawnBiomeType {
    Default,
    USerDefined,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum SpawnType {
    Player,
    World,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum SubChunkRequestMode {
    Legacy,
    Limitless,
    Limited,
}

#[derive(Debug, PartialEq, FromPrimitive, ToPrimitive)]
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

#[derive(Debug, PartialEq, FromPrimitive, ToPrimitive)]
pub enum HeightMapType {
    None,
    HasData,
    TooHigh,
    TooLow,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum GameType {
    Survival,
    Creative,
    Adventure,
    SurvivalSpectator,
    CreativeSpectator,
    Default,
    Spectator,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum Generator {
    Legacy,
    Overworld,
    Flat,
    Nether,
    End,
    Void,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum PermissionLevel {
    Visitor,
    Member,
    Operator,
    Custom,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum EntityLinkType {
    Remove,
    Rider,
    Passenger,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum UpdateBlockTransition {
    BlockToEntity,
    EntityToBlock,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct GenerationFeature {
    name: String,
    json: Bytes,
}

impl GenerationFeature {
    pub fn write(&self, writer: &mut Writer) {
        writer.string(self.name.as_str());
        writer.byte_slice(&self.json);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            name: reader.string(),
            json: reader.byte_slice(),
        }
    }
}

#[derive(Debug)]
pub struct DimensionDefinition {
    name: String,
    range: [i32; 2],
    generator: i32,
}

impl DimensionDefinition {
    pub fn write(&self, writer: &mut Writer) {
        writer.string(self.name.as_str());
        writer.var_i32(self.range[0]);
        writer.var_i32(self.range[1]);
        writer.var_i32(self.generator);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            name: reader.string(),
            range: [reader.var_i32(), reader.var_i32()],
            generator: reader.var_i32(),
        }
    }
}

#[derive(Debug)]
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
        writer.u8(num::ToPrimitive::to_u8(&self.link_type).unwrap());
        writer.bool(self.immediate);
        writer.bool(self.rider_initiated);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            ridden_entity_unique_id: reader.i64(),
            rider_entity_unique_id: reader.i64(),
            link_type: num::FromPrimitive::from_u8(reader.u8()).unwrap(),
            immediate: reader.bool(),
            rider_initiated: reader.bool(),
        }
    }
}

#[derive(Debug)]
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
        writer.u8(num::ToPrimitive::to_u8(&self.result).unwrap());
        if self.result != SubChunkResult::SuccessAllAir || cache_enabled {
            writer.byte_slice(&self.raw_payload);
        }
        writer.u8(num::ToPrimitive::to_u8(&self.height_map_type).unwrap());
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
            result: num::FromPrimitive::from_u8(reader.u8()).unwrap(),
            raw_payload: Bytes::default(),
            height_map_type: HeightMapType::None,
            height_map_data: [0; 256],
            blob_hash: 0,
        };
        if entry.result != SubChunkResult::SuccessAllAir || cache_enabled {
            entry.raw_payload = reader.byte_slice();
        }
        entry.height_map_type = num::FromPrimitive::from_u8(reader.u8()).unwrap();
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

#[derive(Debug)]
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

#[derive(Debug)]
pub struct CacheBlob {
    pub hash: u64,
    pub payload: Bytes,
}

impl CacheBlob {
    pub fn write(&self, writer: &mut Writer) {
        writer.u64(self.hash);
        writer.byte_slice(&self.payload);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            hash: reader.u64(),
            payload: reader.byte_slice(),
        }
    }
}

#[derive(Debug)]
pub struct ExperimentData {
    pub name: String,
    pub enabled: bool,
}

impl ExperimentData {
    pub fn write(&self, writer: &mut Writer) {
        writer.string(self.name.as_str());
        writer.bool(self.enabled);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            name: reader.string(),
            enabled: reader.bool(),
        }
    }
}
