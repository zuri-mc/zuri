use zuri_net_derive::packet;
use crate::proto::io::{Readable, Reader, Writable, Writer};

#[packet]
#[derive(Debug, Clone)]
pub struct EducationExternalLinkSettings {
    pub url: String,
    pub display_name: String,
}

#[derive(Debug, Clone)]
pub struct EducationSharedResourceURI {
    pub button_name: String,
    pub link_uri: String,
}

impl EducationSharedResourceURI {
    pub fn write(&self, writer: &mut Writer) {
        writer.string(self.button_name.as_str());
        writer.string(self.link_uri.as_str());
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            button_name: reader.string(),
            link_uri: reader.string(),
        }
    }
}