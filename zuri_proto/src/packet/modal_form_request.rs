use bytes::Bytes;

use crate::io::{Reader, Writer};
use crate::packet::Packet;

#[derive(Debug)]
pub struct ModalFormRequest {
    pub form_id: u32,
    pub form_data: Bytes,
}

impl Packet for ModalFormRequest {
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
