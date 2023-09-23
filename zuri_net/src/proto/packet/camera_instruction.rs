use crate::proto::ints::VarU32;
use crate::proto::types::colour::RGB;
use glam::{Vec2, Vec3};
use zuri_net_derive::proto;

/// Gives a custom camera specific instructions to operate.
#[proto]
#[derive(Debug, Clone)]
pub struct CameraInstruction {
    /// A compound tag of the instructions to sent. The structure of this tag is currently unknown.
    #[len_type(VarU32)]
    pub data: Vec<CameraInstructionEntry>,
}

#[proto]
#[derive(Debug, Clone)]
pub struct CameraInstructionEntry {
    pub set: Option<CameraInstructionSet>,
    pub clear: Option<bool>,
    pub fade: Option<CameraInstructionFade>,
}

#[proto]
#[derive(Debug, Clone)]
pub struct CameraInstructionSet {
    pub preset: u32,
    pub ease: Option<CameraEase>,
    pub position: Option<Vec3>,
    pub rotation: Option<Vec2>,
    pub facing: Option<Vec3>,
    pub default: Option<bool>,
}

#[proto]
#[derive(Debug, Clone)]
pub struct CameraEase {
    pub r#type: CameraEaseType,
    pub duration: f32,
}

#[proto(u8)]
#[derive(Debug, Clone)]
pub enum CameraEaseType {
    EasingTypeLinear,
    EasingTypeSpring,
    EasingTypeInQuad,
    EasingTypeOutQuad,
    EasingTypeInOutQuad,
    EasingTypeInCubic,
    EasingTypeOutCubic,
    EasingTypeInOutCubic,
    EasingTypeInQuart,
    EasingTypeOutQuart,
    EasingTypeInOutQuart,
    EasingTypeInQuint,
    EasingTypeOutQuint,
    EasingTypeInOutQuint,
    EasingTypeInSine,
    EasingTypeOutSine,
    EasingTypeInOutSine,
    EasingTypeInExpo,
    EasingTypeOutExpo,
    EasingTypeInOutExpo,
    EasingTypeInCirc,
    EasingTypeOutCirc,
    EasingTypeInOutCirc,
    EasingTypeInBounce,
    EasingTypeOutBounce,
    EasingTypeInOutBounce,
    EasingTypeInBack,
    EasingTypeOutBack,
    EasingTypeInOutBack,
    EasingTypeInElastic,
    EasingTypeOutElastic,
    EasingTypeInOutElastic,
}

#[proto(u8)]
#[derive(Debug, Clone)]
pub struct CameraInstructionFade {
    pub fade_in_duration: f32,
    pub wait_duration: f32,
    pub fade_out_duration: f32,
    pub colour: RGB,
}
