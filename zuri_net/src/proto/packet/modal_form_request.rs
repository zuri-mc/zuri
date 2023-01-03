use bytes::Bytes;

use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent by the server to make the client open a form. This form may be either a modal form which
/// has two options, a menu form for a selection of options and a custom form for properties.
#[derive(Debug, Clone)]
pub struct ModalFormRequest {
    /// An ID used to identify the form. The ID is saved by the client and sent back when the player
    /// submits the form, so that the server can identify which form was submitted.
    pub form_id: u32,
    /// JSON encoded object of form data. The content of the object differs, depending on the type
    /// of the form sent, which is also set in the JSON.
    pub form_data: Bytes,
}

impl PacketType for ModalFormRequest {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.form_id);
        writer.byte_slice(&self.form_data);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            form_id: reader.var_u32(),
            form_data: reader.byte_slice(),
        }
    }
}
