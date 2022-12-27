use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::packet::PacketType;
use crate::proto::io::{Reader, Writer};

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum PositionTrackingDBRequestAction {
    Query
}

#[derive(Debug)]
pub struct PositionTrackingDBClientRequest {
    pub request_action: PositionTrackingDBRequestAction,
    pub tracking_id: i32,
}

impl PacketType for PositionTrackingDBClientRequest {
    fn write(&self, writer: &mut Writer) {
        writer.u8(self.request_action.to_u8().unwrap());
        writer.var_i32(self.tracking_id);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            request_action: PositionTrackingDBRequestAction::from_u8(reader.u8()).unwrap(),
            tracking_id: reader.var_i32(),
        }
    }
}
