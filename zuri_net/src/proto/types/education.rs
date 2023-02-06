use zuri_net_derive::packet;

#[packet]
#[derive(Debug, Clone)]
pub struct EducationExternalLinkSettings {
    pub url: String,
    pub display_name: String,
}

#[packet]
#[derive(Debug, Clone)]
pub struct EducationSharedResourceURI {
    pub button_name: String,
    pub link_uri: String,
}
