use crate::proto::ints::{VarI32, VarI64};
use glam::Vec3;
use zuri_net_derive::proto;

use crate::proto::io::UBlockPos;

#[proto(VarI32)]
#[derive(Debug, Clone)]
pub enum StructureBlockType {
    Data,
    Save,
    Load,
    Corner,
    Invalid,
    Export,
}

#[derive(Debug, Clone)]
pub enum StructureMirrorAxis {
    None,
    X,
    Z,
    Both,
}

#[proto(VarI32)]
#[derive(Debug, Clone)]
pub enum StructureRedstoneSaveMode {
    Memory,
    Disk,
}

#[derive(Debug, Clone)]
pub enum StructureRotation {
    None,
    Rotate90,
    Rotate180,
    Rotate270,
}

#[proto(u8)]
#[derive(Debug, Clone)]
pub enum StructureTemplateDataRequestType {
    None,
    ExportFromSave,
    ExportFromLoad,
    QuerySavedStructure,
    ImportFromSave,
}

#[derive(Debug, Clone)]
pub enum StructureTemplateDataResponseType {
    Export,
    Query,
    Import,
}

#[proto(u8)]
#[derive(Debug, Clone)]
pub enum AnimationMode {
    None,
    Layers,
    Blocks,
}

#[proto]
#[derive(Debug, Clone)]
pub struct StructureSettings {
    pub palette_name: String,
    pub ignore_entities: bool,
    pub ignore_blocks: bool,
    pub allow_non_ticking_chunks: bool,
    pub size: UBlockPos,
    pub offset: UBlockPos,
    pub last_editing_player_unique_id: VarI64,
    pub rotation: u8,
    pub mirror: u8,
    pub animation_mode: AnimationMode,
    pub animation_duration: f32,
    pub integrity: f32,
    pub seed: u32,
    pub pivot: Vec3,
}
