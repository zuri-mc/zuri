use bytes::Bytes;

use zuri_net_derive::proto;

use crate::proto::ints::VarI32;

/// Sent by the server to send a 'generic' level event to the client. This packet sends an NBT
/// serialised object and may for that reason be used for any event holding additional data.
#[proto]
#[derive(Debug, Clone)]
pub struct LevelEventGeneric {
    /// A unique identifier that identifies the event called. The data that follows has fields in
    /// the NBT depending on what event it is.
    pub event_id: VarI32,
    /// A network little endian serialised object of event data, with fields that vary depending on
    /// the event ID. Unlike many other NBT structures, this data is not actually in a compound but
    /// just loosely floating NBT tags. To decode using the nbt package, you would need to append
    /// 0x0a00 at the start (compound ID and name length) and add 0x00 at the end, to manually wrap
    /// it in a compound. Likewise, you would have to remove these bytes when encoding.
    pub serialised_event_data: Bytes,
}
