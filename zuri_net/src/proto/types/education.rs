use crate::proto::io::{Read, Reader, Write, Writer};

#[derive(Debug, Clone)]
pub struct EducationExternalLinkSettings {
    pub url: String,
    pub display_name: String,
}

impl EducationExternalLinkSettings {
    pub fn write(&self, writer: &mut Writer) {
        writer.string(self.url.as_str());
        writer.string(self.display_name.as_str());
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            url: reader.string(),
            display_name: reader.string(),
        }
    }
}

impl Write for EducationExternalLinkSettings {
    fn write(&self, writer: &mut Writer) {
        self.write(writer)
    }
}

impl Read<EducationExternalLinkSettings> for EducationExternalLinkSettings {
    fn read(reader: &mut Reader) -> EducationExternalLinkSettings {
        EducationExternalLinkSettings::read(reader)
    }
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