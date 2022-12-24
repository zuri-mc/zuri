use bytes::Bytes;
use num_derive::{FromPrimitive, ToPrimitive};

use crate::packet::Packet;
use crate::io::{Reader, Writer};

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum ModalFormCancelReason {
    UserClosed,
    UserBusy,
}

#[derive(Debug)]
pub struct ModalFormResponse {
    pub form_id: u32,
    pub response_data: Option<Bytes>,
    pub cancel_reason: Option<ModalFormCancelReason>,
}

impl Packet for ModalFormResponse {
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
