use bytes::Bytes;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::io::{Readable, Reader, Writable, Writer};
use crate::proto::packet::PacketType;

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum ModalFormCancelReason {
    UserClosed,
    UserBusy,
}

/// Sent by the client in response to a ModalFormRequest, after the player has submitted the form
/// sent. It contains the options/properties selected by the player, or a JSON encoded 'null' if the
/// form was closed by clicking the X at the top right corner of the form.
#[derive(Debug, Clone)]
pub struct ModalFormResponse {
    /// The form ID of the form the client has responded to. It is the same as the ID sent in the
    /// ModalFormRequest, and may be used to identify which form was submitted.
    pub form_id: u32,
    /// JSON encoded value representing the response of the player. For a modal form, the response
    /// is either true or false, for a menu form, the response is an integer specifying the index of
    /// the button clicked, and for a custom form, the response is an array containing a value for
    /// each element.
    pub response_data: Option<Bytes>,
    /// The reason why the form was cancelled.
    pub cancel_reason: Option<ModalFormCancelReason>,
}

impl PacketType for ModalFormResponse {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.form_id);
        writer.optional(&self.response_data);
        writer.optional(&self.cancel_reason);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            form_id: reader.var_u32(),
            response_data: reader.optional(),
            cancel_reason: reader.optional(),
        }
    }
}

impl Writable for ModalFormCancelReason {
    fn write(&self, writer: &mut Writer) {
        writer.u8(self.to_u8().unwrap())
    }
}

impl Readable<ModalFormCancelReason> for ModalFormCancelReason {
    fn read(reader: &mut Reader) -> ModalFormCancelReason {
        ModalFormCancelReason::from_u8(reader.u8()).unwrap()
    }
}
