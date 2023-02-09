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
    TVOS,
    Orbis,
    NX,
    XBOX,
    WP,
    Linux,
    #[fallback]
    #[serde(other)]
    Unknown,
}
