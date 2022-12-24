/// Sent by the server to initiate a certain event that has to do with blocks in specific, for example opening chests.
#[derive(Debug)]
pub struct BlockEvent {
    /// The position of the block that an event occurred at.
    pub position: BlockPos,
    /// The type of the block event. The event type decides the way the event data that follows is used.
    pub event_type: BlockEventType,
    /// Holds event type specific data. For chests, for example, opening the chest means the data must hold one, whereas
    /// closing it should hold zero.
    pub event_data: i32,
}

impl Packet for BlockEvent {
    fn write(&self, writer: &mut Writer) {
        writer.u_block_pos(self.position);
        writer.var_i32(num::ToPrimitive::to_i32(&self.event_type).unwrap());
        writer.var_i32(self.event_data);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            position: reader.u_block_pos(),
            event_type: num::FromPrimitive::from_i32(reader.var_i32()).unwrap(),
            event_data: reader.var_i32(),
        }
    }
}
