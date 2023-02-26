use zuri_net_derive::proto;

/// Sent by the server to make the camera shake client-side. This feature was added for map-making
/// partners.
#[proto]
#[derive(Debug, Clone)]
pub struct CameraShake {
    /// The intensity of the shaking. The client limits this value to 4, so anything higher may not
    /// function, at least as expected.
    pub intensity: f32,
    /// The number of seconds the camera will shake for.
    pub duration: f32,
    /// The type of shake. The different type affects how the shake looks in game.
    pub shake_type: CameraShakeType,
    /// The action to be performed. Currently, the different actions will either add or stop shaking
    /// the camera client-side.
    pub action: CameraShakeAction,
}

#[proto(u8)]
#[derive(Debug, Clone)]
pub enum CameraShakeAction {
    Add,
    Stop,
}

#[proto(u8)]
#[derive(Debug, Clone)]
pub enum CameraShakeType {
    Positional,
    Rotational,
}
