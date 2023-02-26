use zuri_net_derive::proto;

use crate::proto::ints::VarI64;

/// Sent by the client to request photo information from the server.
#[proto]
#[derive(Debug, Clone)]
pub struct PhotoInfoRequest {
    /// The ID of the photo.
    pub photo_id: VarI64,
}
