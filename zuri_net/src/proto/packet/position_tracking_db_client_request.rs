use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::packet::PacketType;
use crate::proto::io::{Reader, Writer};

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum PositionTrackingDBRequestAction {
    Query
}

/// Sent by the client to request the position and dimension of a 'tracking ID'. These IDs are
/// tracked in a database by the server. In 1.16, this is used for lodestones. The client will send
/// this request to find the position a lodestone compass needs to point to. If found, it will point
/// to the lodestone. If not, it will start spinning around. A PositionTrackingDBServerBroadcast
/// packet should be sent in response to this packet.
#[derive(Debug, Clone)]
pub struct PositionTrackingDBClientRequest {
    /// The action that should be performed upon the receiving of the packet.
    pub request_action: PositionTrackingDBRequestAction,
    /// A unique ID used to identify the request. The server responds with a
    /// PositionTrackingDBServerBroadcast packet holding the same ID, so that the client can find
    /// out what that packet was in response to.
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
