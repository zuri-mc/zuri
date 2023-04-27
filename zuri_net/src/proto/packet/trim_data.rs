use crate::proto::ints::VarU32;
use zuri_net_derive::proto;

#[proto]
#[derive(Debug, Clone)]
pub struct TrimData {
    #[len_type(VarU32)]
    pub patterns: Vec<TrimPattern>,
    #[len_type(VarU32)]
    pub materials: Vec<TrimMaterial>,
}

#[proto]
#[derive(Debug, Clone)]
pub struct TrimPattern {
    pub item_name: String,
    pub pattern_id: String,
}

#[proto]
#[derive(Debug, Clone)]
pub struct TrimMaterial {
    pub material_id: String,
    pub colour: String,
    pub item_name: String,
}
