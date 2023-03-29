use crate::proto::ints::VarU32;
use bytes::Bytes;
use zuri_net_derive::proto;

/// Sent by the client in response to a ModalFormRequest, after the player has submitted the form
/// sent. It contains the options/properties selected by the player, or a JSON encoded 'null' if the
/// form was closed by clicking the X at the top right corner of the form.
#[proto]
#[derive(Debug, Clone)]
pub struct ModalFormResponse {
    /// The form ID of the form the client has responded to. It is the same as the ID sent in the
    /// ModalFormRequest, and may be used to identify which form was submitted.
    pub form_id: VarU32,
    /// JSON encoded value representing the response of the player. For a modal form, the response
    /// is either true or false, for a menu form, the response is an integer specifying the index of
    /// the button clicked, and for a custom form, the response is an array containing a value for
    /// each element.
    pub response_data: Option<Bytes>,
    /// The reason why the form was cancelled.
    pub cancel_reason: Option<ModalFormCancelReason>,
}

#[proto(u8)]
#[derive(Debug, Clone)]
pub enum ModalFormCancelReason {
    UserClosed,
    UserBusy,
}
