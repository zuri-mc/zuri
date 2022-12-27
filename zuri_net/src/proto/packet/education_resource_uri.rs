use crate::proto::packet::PacketType;
use crate::proto::io::{Reader, Writer};
use crate::proto::types::education::EducationSharedResourceURI;

#[derive(Debug, Clone)]
pub struct EducationResourceURI {
    pub resource: EducationSharedResourceURI,
}

impl PacketType for EducationResourceURI {
    fn write(&self, writer: &mut Writer) {
        self.resource.write(writer);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            resource: EducationSharedResourceURI::read(reader),
        }
    }
}
