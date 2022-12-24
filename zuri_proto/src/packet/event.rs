use crate::io::{Reader, Writer};
use crate::packet::Packet;
use crate::types::event::EventType;

#[derive(Debug)]
pub struct Event {
    pub entity_runtime_id: u64,
    pub use_player_id: u8,
    pub event_data: EventType,
}

impl Packet for Event {
    fn write(&self, writer: &mut Writer) {
        writer.var_u64(self.entity_runtime_id);
        // todo: THIS DOESNT WORK BECAUSE OF THE FUCKING USE_PLAYER_ID
        //writer.var_i32(num::ToPrimitive::to_i32(&self.event_data.event_type()).unwrap());
        writer.u8(self.use_player_id);
        //self.event_data.write(writer);
    }

    fn read(reader: &mut Reader) -> Self {
        let entity_runtime_id = reader.var_u64();
        // todo: FUCJKING USE_POLAYER_ID @#G@O*GF)*@GV#
        //let event_type = num::FromPrimitive::from_i32(reader.var_i32()).unwrap();
        Self {
            entity_runtime_id,
            use_player_id: reader.u8(),
            event_data: EventType::read(reader),
        }
    }
}
