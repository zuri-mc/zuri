use zuri_net_derive::proto;

use crate::proto::ints::VarU32;
use crate::proto::types::world::GenerationFeature;

/// Notifies the client about the world generation features the server is currently using. This is
/// used in combination with the client-side world generation system introduced in v1.19.20,
/// allowing the client to completely generate the chunks of the world without having to rely on the
/// server.
#[proto]
#[derive(Debug, Clone)]
pub struct FeatureRegistry {
    /// A list of all registered world generation features.
    #[len_type(VarU32)]
    pub features: Vec<GenerationFeature>,
}
