use zuri_net_derive::proto;

use crate::proto::types::resource_pack::ResourcePackResponse;

/// Sent by the client in response to resource packets sent by the server. It is used to let the
/// server know what action needs to be taken for the client to have all resource packs ready.
#[proto]
#[derive(Debug, Clone)]
pub struct ResourcePackClientResponse {
    /// The response type the client gave.
    pub response: ResourcePackResponse,
    /// A list of resource pack UUIDs combined with their version that need to be downloaded, if the
    /// `response` field is `SendPacks`.
    #[len_type(u16)]
    pub packs_to_download: Vec<String>,
}
