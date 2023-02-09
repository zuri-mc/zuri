use bytes::Bytes;

use zuri_net_derive::proto;

use crate::proto::ints::VarU32;

/// Optionally sent by the server in response to a ServerSettingsRequest from the client. It is
/// structured the same as a ModalFormRequest packet, and if filled out correctly, will show a
/// specific tab for the server in the settings of the client. A ModalFormResponse packet is sent by
/// the client in response to a ServerSettingsResponse, when the client fills out the settings and
/// closes the settings again.
#[proto]
#[derive(Debug, Clone)]
pub struct ServerSettingsResponse {
    /// An ID used to identify the form. The ID is saved by the client and sent back when the player
    /// submits the form, so that the server can identify which form was submitted.
    pub form_id: VarU32,
    /// JSON encoded object of form data. The content of the object differs, depending on the type
    /// of the form sent, which is also set in the JSON.
    pub form_data: Bytes,
}
