use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

use crate::packet::PacketType;
use crate::io::{Reader, Writer};

#[derive(Debug)]
pub struct SimulationType {
    pub simulation_type: Simulation,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum Simulation {
    Game,
    Editor,
    Test,
    Invalid,
}

impl PacketType for SimulationType {
    fn write(&self, writer: &mut Writer) {
        writer.u8(self.simulation_type.to_u8().unwrap());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            simulation_type: Simulation::from_u8(reader.u8()).unwrap(),
        }
    }
}
