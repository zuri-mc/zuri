use zuri_net_derive::proto;

use crate::proto::types::education::EducationExternalLinkSettings;

/// Sent by the server to update Education Edition related settings. It is unused by vanilla.
#[proto]
#[derive(Debug, Clone)]
pub struct EducationSettings {
    /// The default URI that the code builder is ran on. Using this, a Code Builder program can
    /// make code directly affect the server.
    pub code_builder_default_uri: String,
    /// The title of the code builder shown when connected to the code builder default URI.
    pub code_builder_title: String,
    /// Specifies if clients connected to the world should be able to resize the code builder when
    /// it is opened.
    pub can_resize_code_builder: bool,
    /// The purpose of this field is currently unknown.
    pub disable_legacy_title_bar: bool,
    /// The purpose of this field is currently unknown.
    pub post_process_filter: String,
    /// The purpose of this field is currently unknown.
    pub screenshot_border_path: String,
    /// The purpose of this field is currently unknown.
    pub can_modify_blocks: Option<bool>,
    /// The purpose of this field is currently unknown.
    pub override_uri: Option<String>,
    /// Specifies if the world has a quiz connected to it.
    pub has_quiz: bool,
    /// The purpose of this field is currently unknown.
    pub external_link_settings: Option<EducationExternalLinkSettings>,
}
