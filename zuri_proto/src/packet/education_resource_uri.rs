use crate::io::{Reader, Writer};
use crate::packet::Packet;

#[derive(Debug)]
pub struct EducationResourceURI {
    pub resource: EducationSharedResourceURI,
}

impl Packet for EducationResourceURI {
    fn write(&self, writer: &mut Writer) {
        self.resource.write(writer);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            resource: EducationSharedResourceURI::read(reader),
        }
    }
}
