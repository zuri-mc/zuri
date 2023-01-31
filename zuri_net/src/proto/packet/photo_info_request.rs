use zuri_net_derive::packet;

use crate::proto::ints::VarI64;

/// Sent by the client to request photo information from the server.
#[packet]
#[derive(Debug, Clone)]
pub struct PhotoInfoRequest {
    /// The ID of the photo.
    pub photo_id: VarI64,
}
