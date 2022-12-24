use crate::io::{Reader, Writer};
use crate::packet::Packet;

#[derive(Debug)]
pub struct SimulationType {
    pub simulation_type: Simulation,
}

impl Packet for SimulationType {
    fn write(&self, writer: &mut Writer) {
        writer.u8(num::ToPrimitive::to_u8(&self.simulation_type).unwrap());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            simulation_type: num::FromPrimitive::from_u8(reader.u8()).unwrap(),
        }
    }
}
