use glam::{IVec3, Vec3};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{ToPrimitive, FromPrimitive};
use crate::io::{Reader, Writer};

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum StructureBlockType {
    Data,
    Save,
    Load,
    Corner,
    Invalid,
    Export,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum StructureMirrorAxis {
    None,
    X,
    Z,
    Both,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum StructureRedstoneSaveMode {
    Memory,
    Disk,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum StructureRotation {
    None,
    Rotate90,
    Rotate180,
    Rotate270,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum StructureTemplateDataRequestType {
    None,
    ExportFromSave,
    ExportFromLoad,
    QuerySavedStructure,
    ImportFromSave,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum StructureTemplateDataResponseType {
    Export,
    Query,
    Import,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum AnimationMode {
    None,
    Layers,
    Blocks,
}

#[derive(Debug)]
pub struct StructureSettings {
    pub palette_name: String,
    pub ignore_entities: bool,
    pub ignore_blocks: bool,
    pub allow_non_ticking_chunks: bool,
    pub size: IVec3,
    pub offset: IVec3,
    pub last_editing_player_unique_id: i64,
    pub rotation: u8,
    pub mirror: u8,
    pub animation_mode: AnimationMode,
    pub animation_duration: f32,
    pub integrity: f32,
    pub seed: u32,
    pub pivot: Vec3,
}

impl StructureSettings {
    pub fn write(&self, writer: &mut Writer) {
        writer.string(self.palette_name.as_str());
        writer.bool(self.ignore_entities);
        writer.bool(self.ignore_blocks);
        writer.bool(self.allow_non_ticking_chunks);
        writer.u_block_pos(self.size);
        writer.u_block_pos(self.offset);
        writer.var_i64(self.last_editing_player_unique_id);
        writer.u8(self.rotation);
        writer.u8(self.mirror);
        writer.u8(self.animation_mode.to_u8().unwrap());
        writer.f32(self.animation_duration);
        writer.f32(self.integrity);
        writer.u32(self.seed);
        writer.vec3(self.pivot);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            palette_name: reader.string(),
            ignore_entities: reader.bool(),
            ignore_blocks: reader.bool(),
            allow_non_ticking_chunks: reader.bool(),
            size: reader.u_block_pos(),
            offset: reader.u_block_pos(),
            last_editing_player_unique_id: reader.var_i64(),
            rotation: reader.u8(),
            mirror: reader.u8(),
            animation_mode: AnimationMode::from_u8(reader.u8()).unwrap(),
            animation_duration: reader.f32(),
            integrity: reader.f32(),
            seed: reader.u32(),
            pivot: reader.vec3(),
        }
    }
}

