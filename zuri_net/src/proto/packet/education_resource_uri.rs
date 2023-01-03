use crate::proto::packet::PacketType;
use crate::proto::io::{Reader, Writer};
use crate::proto::types::education::EducationSharedResourceURI;

/// Transmits education resource settings to all clients.
#[derive(Debug, Clone)]
pub struct EducationResourceURI {
    /// The resource that is being referenced.
    pub resource: EducationSharedResourceURI,
}

impl PacketType for EducationResourceURI {
    fn write(&self, writer: &mut Writer) {
        self.resource.write(writer);
    }

    fn read(reader: &mut Reader) -> Self {
        Self { resource: EducationSharedResourceURI::read(reader) }
    }
}
