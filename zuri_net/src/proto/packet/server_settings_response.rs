use bytes::Bytes;
use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Optionally sent by the server in response to a ServerSettingsRequest from the client. It is
/// structured the same as a ModalFormRequest packet, and if filled out correctly, will show a
/// specific tab for the server in the settings of the client. A ModalFormResponse packet is sent by
/// the client in response to a ServerSettingsResponse, when the client fills out the settings and
/// closes the settings again.
#[derive(Debug, Clone)]
pub struct ServerSettingsResponse {
    /// An ID used to identify the form. The ID is saved by the client and sent back when the player
    /// submits the form, so that the server can identify which form was submitted.
    pub form_id: u32,
    /// JSON encoded object of form data. The content of the object differs, depending on the type
    /// of the form sent, which is also set in the JSON.
    pub form_data: Bytes,
}

impl PacketType for ServerSettingsResponse {
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
