use num_derive::{FromPrimitive, ToPrimitive};
use zuri_net_derive::proto;

#[proto(u8)]
#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum ResourcePackResponse {
    None,
    Refused,
    SendPacks,
    AllPacksDownloaded,
    Completed,
}

#[proto(u8)]
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
