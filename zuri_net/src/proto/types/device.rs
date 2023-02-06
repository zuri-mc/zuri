use num_derive::{FromPrimitive, ToPrimitive};
use serde_repr::{Deserialize_repr, Serialize_repr};
use zuri_net_derive::packet;

#[packet(i32)]
#[repr(i32)]
#[derive(Debug, Copy, Clone, FromPrimitive, ToPrimitive, Serialize_repr, Deserialize_repr)]
pub enum Device {
    None,
    Android,
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
}
