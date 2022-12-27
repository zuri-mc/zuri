use crate::proto::packet::PacketType;
use crate::proto::io::{Reader, Writer};
use crate::proto::types::education::EducationExternalLinkSettings;

#[derive(Debug)]
pub struct EducationSettings {
    pub code_builder_default_uri: String,
    pub code_builder_title: String,
    pub can_resize_code_builder: bool,
    pub disable_legacy_title_bar: bool,
    pub post_process_filter: String,
    pub screenshot_border_path: String,
    pub can_modify_blocks: Option<bool>,
    pub override_uri: Option<String>,
    pub has_quiz: bool,
    pub external_link_settings: Option<EducationExternalLinkSettings>,
}

impl PacketType for EducationSettings {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.code_builder_default_uri.as_str());
        writer.string(self.code_builder_title.as_str());
        writer.bool(self.can_resize_code_builder);
        writer.bool(self.disable_legacy_title_bar);
        writer.string(self.post_process_filter.as_str());
        writer.string(self.screenshot_border_path.as_str());
        writer.optional(&self.can_modify_blocks);
        writer.optional(&self.override_uri);
        writer.bool(self.has_quiz);
        writer.optional(&self.external_link_settings);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            code_builder_default_uri: reader.string(),
            code_builder_title: reader.string(),
            can_resize_code_builder: reader.bool(),
            disable_legacy_title_bar: reader.bool(),
            post_process_filter: reader.string(),
            screenshot_border_path: reader.string(),
            can_modify_blocks: reader.optional(),
            override_uri: reader.optional(),
            has_quiz: reader.bool(),
            external_link_settings: reader.optional(),
        }
    }
}
