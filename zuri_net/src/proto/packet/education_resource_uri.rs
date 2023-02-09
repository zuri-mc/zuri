use zuri_net_derive::proto;

use crate::proto::types::education::EducationSharedResourceURI;

/// Transmits education resource settings to all clients.
#[proto]
#[derive(Debug, Clone)]
pub struct EducationResourceURI {
    /// The resource that is being referenced.
    pub resource: EducationSharedResourceURI,
}
