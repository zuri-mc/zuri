use zuri_net_derive::packet;
use crate::proto::packet::PacketType;
use crate::proto::types::education::EducationSharedResourceURI;

/// Transmits education resource settings to all clients.
#[packet]
#[derive(Debug, Clone)]
pub struct EducationResourceURI {
    /// The resource that is being referenced.
    pub resource: EducationSharedResourceURI,
}
