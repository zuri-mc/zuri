use std::collections::{BTreeMap, HashMap};
use bevy::prelude::Vec3;
use serde::{Serialize, Deserialize, Deserializer};
use serde_json::{to_string, Value};

#[derive(Serialize, Deserialize, Debug)]
pub struct AnimationList {
    pub format_version: String,
    pub animations: HashMap<String, Animation>,
}

impl AnimationList {
    pub fn new(data: &str) -> HashMap<String, Animation> {
        let mut list: AnimationList = serde_json::from_str(data).unwrap();
        list.animations
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Animation {
    #[serde(rename = "loop", default, deserialize_with = "deserialize_loop_mode")]
    pub loop_mode: LoopMode,
    pub animation_length: f32,
    pub bones: BTreeMap<String, BoneAnimation>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BoneAnimation {
    pub rotation: Option<BTreeMap<String, Value>>,
    pub position: Option<BTreeMap<String, Value>>,
    pub scale: Option<BTreeMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StepInterpolation {
    pub pre: BTreeMap<String, Value>,
    pub post: BTreeMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SmoothInterpolation {
    pub pre: BTreeMap<String, Value>,
    pub post: BTreeMap<String, Value>,
    pub lerp_mode: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub enum LoopMode {
    Loop,
    #[default]
    Once,
    Hold,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Interpolation {
    Linear(BTreeMap<String, Value>),
    Smooth(SmoothInterpolation),
    Step(StepInterpolation),
    // TODO: https://bedrock.dev/docs/stable/Animations#Entity%20Animation%20Format%20Examples
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AnimationValue {
    SingleAxis(f32),
    SingleAxisArray([f32; 1]),
    AllAxes([f32; 3]),
}

impl Into<Vec3> for AnimationValue {
    fn into(self) -> Vec3 {
        match self {
            AnimationValue::SingleAxis(value) => Vec3::new(value, value, value),
            AnimationValue::SingleAxisArray([value]) => Vec3::new(value, value, value),
            AnimationValue::AllAxes([x, y, z]) => Vec3::new(x, y, z),
        }
    }
}

fn deserialize_loop_mode<'de, D>(deserializer: D) -> Result<LoopMode, D::Error>
    where D: Deserializer<'de>,
{
    match Value::deserialize(deserializer)? {
        Value::Bool(b) => Ok(if b { LoopMode::Loop } else { LoopMode::Once }),
        Value::String(s) => {
            if s == "hold_on_last_frame" {
                Ok(LoopMode::Hold)
            } else {
                Err(serde::de::Error::custom("invalid loop mode"))
            }
        }
        _ => Err(serde::de::Error::custom("invalid loop mode")),
    }
}