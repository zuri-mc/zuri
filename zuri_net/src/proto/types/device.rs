#![allow(deprecated)]

use serde_repr::{Deserialize_repr, Serialize_repr};
use zuri_net_derive::proto;

#[proto(i32)]
#[repr(i32)]
#[derive(Debug, Copy, Clone, Serialize_repr, Deserialize_repr)]
pub enum Device {
    Android = 1,
    IOS,
    OSX,
    FireOS,
    GearVR,
    Hololens,
    Win10,
    Win32,
    Dedicated,
    #[deprecated = "Deprecated as of Bedrock Edition v1.20.10"]
    TVOS,
    Orbis,
    NX,
    XBOX,
    #[deprecated = "Deprecated as of Bedrock Edition v1.20.10"]
    WP,
    Linux,
    #[fallback]
    #[serde(other)]
    Unknown,
}
