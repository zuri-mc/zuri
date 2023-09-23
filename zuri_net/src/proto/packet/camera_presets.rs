use crate::proto::ints::VarU32;
use zuri_net_derive::proto;

/// Gives the client a list of custom camera presets.
#[proto]
#[derive(Debug, Clone)]
pub struct CameraPresets {
    /// A compound tag of the presets to sent. The structure of this tag is currently unknown.
    #[len_type(VarU32)]
    pub data: Vec<CameraPresetEntry>,
}

#[proto]
#[derive(Debug, Clone)]
pub struct CameraPresetEntry {
    pub name: String,
    pub parent: String,
    pub pos_x: Option<f32>,
    pub pos_y: Option<f32>,
    pub pos_z: Option<f32>,
    pub rot_x: Option<f32>,
    pub rot_y: Option<f32>,
    pub listener: Option<AudioListener>,
    pub player_effects: Option<bool>,
}

#[proto(u8)]
#[derive(Debug, Clone)]
pub enum AudioListener {
    Camera,
    Player,
}
