use zuri_net_derive::proto;

#[proto]
#[derive(Debug, Clone)]
pub struct EducationExternalLinkSettings {
    pub url: String,
    pub display_name: String,
}

#[proto]
#[derive(Debug, Clone)]
pub struct EducationSharedResourceURI {
    pub button_name: String,
    pub link_uri: String,
}
