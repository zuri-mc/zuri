use bytes::Bytes;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{ToPrimitive, FromPrimitive};

use crate::proto::packet::PacketType;
use crate::proto::io::{Read, Reader, Write, Writer};

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

impl Write for ModalFormCancelReason {
    fn write(&self, writer: &mut Writer) {
        writer.u8(self.to_u8().unwrap())
    }
}

impl Read<ModalFormCancelReason> for ModalFormCancelReason {
    fn read(reader: &mut Reader) -> ModalFormCancelReason {
        ModalFormCancelReason::from_u8(reader.u8()).unwrap()
    }
}
