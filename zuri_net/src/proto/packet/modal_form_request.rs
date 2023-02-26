use bytes::Bytes;

use zuri_net_derive::proto;

use crate::proto::ints::VarU32;

/// Sent by the server to make the client open a form. This form may be either a modal form which
/// has two options, a menu form for a selection of options and a custom form for properties.
#[proto]
#[derive(Debug, Clone)]
pub struct ModalFormRequest {
    /// An ID used to identify the form. The ID is saved by the client and sent back when the player
    /// submits the form, so that the server can identify which form was submitted.
    pub form_id: VarU32,
    /// JSON encoded object of form data. The content of the object differs, depending on the type
    /// of the form sent, which is also set in the JSON.
    pub form_data: Bytes,
}
