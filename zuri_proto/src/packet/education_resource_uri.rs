use crate::packet::PacketType;
use crate::io::{Reader, Writer};
use crate::types::education::EducationSharedResourceURI;

#[derive(Debug)]
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
