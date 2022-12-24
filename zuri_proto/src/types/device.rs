use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Debug, Copy, Clone, FromPrimitive, ToPrimitive)]
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
