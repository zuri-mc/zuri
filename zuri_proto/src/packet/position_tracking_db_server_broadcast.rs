use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use zuri_nbt::{Value, encoding::NetworkLittleEndian};

use crate::io::{Reader, Writer};
use crate::packet::PacketType;

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum PositionTrackingDBBroadcastAction {
    Update,
    Destroy,
    NotFound,
}

#[derive(Debug)]
pub struct PositionTrackingDBServerBroadcast {
    pub broadcast_action: PositionTrackingDBBroadcastAction,
    pub tracking_id: i32,
    pub payload: Value,
}

impl PacketType for PositionTrackingDBServerBroadcast {
    fn write(&self, writer: &mut Writer) {
        writer.u8(self.broadcast_action.to_u8().unwrap());
        writer.var_i32(self.tracking_id);
        writer.nbt(&self.payload, NetworkLittleEndian);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            broadcast_action: PositionTrackingDBBroadcastAction::from_u8(reader.u8()).unwrap(),
            tracking_id: reader.var_i32(),
            payload: reader.nbt(NetworkLittleEndian),
        }
    }
}
