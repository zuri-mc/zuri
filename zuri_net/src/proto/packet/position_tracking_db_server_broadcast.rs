use crate::proto::ints::VarI32;
use zuri_nbt::encoding::NetworkLittleEndian;
use zuri_net_derive::proto;

use crate::proto::io::NBT;

/// Sent by the server in response to the PositionTrackingDBClientRequest packet. This packet is, as
/// of 1.16, currently only used for lodestones. The server maintains a database with tracking IDs
/// and their position and dimension. The client will request these tracking IDs, (NBT tag set on
/// the lodestone compass with the tracking ID?) and the server will respond with the status of
/// those tracking IDs. What is actually done with the data sent depends on what the client chooses
/// to do with it. For the lodestone compass, it is used to make the compass point towards
/// lodestones and to make it spin if the lodestone at a position is no longer there.\
#[proto]
#[derive(Debug, Clone)]
pub struct PositionTrackingDBServerBroadcast {
    /// Specifies the status of the position tracking DB response. The `Update` action is sent for
    /// setting the position of a lodestone compass, the `Destroy` and `NotFound` to indicate that
    /// there is not (no longer) a lodestone at that position.
    pub broadcast_action: PositionTrackingDBBroadcastAction,
    /// The ID of the PositionTrackingDBClientRequest packet that this packet was in response to.
    /// The tracking ID is also present as the 'id' field in the serialised data field.
    pub tracking_id: VarI32,
    /// A network little endian tag holding the data retrieved from the position tracking DB.
    pub payload: NBT<NetworkLittleEndian>,
}

#[proto(u8)]
#[derive(Debug, Clone)]
pub enum PositionTrackingDBBroadcastAction {
    Update,
    Destroy,
    NotFound,
}
