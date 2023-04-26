use zuri_net_derive::proto;

#[proto(u8)]
#[derive(Debug, Clone, PartialEq)]
pub enum ResourcePackResponse {
    None,
    Refused,
    SendPacks,
    AllPacksDownloaded,
    Completed,
}

#[proto(u8)]
#[derive(Debug, Clone)]
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
