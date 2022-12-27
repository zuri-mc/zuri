use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum ResourcePackResponse {
    None,
    Refused,
    SendPacks,
    AllPacksDownloaded,
    Completed,
}

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum ResourcePackType {
    Addon,
    Cached,
    CopyProtected,
    Behaviour,
    PersonaPiece,
    Resources,
    Skins,
    WorldTemplate,
}
