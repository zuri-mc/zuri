use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum Simulation {
    Game,
    Editor,
    Test,
    Invalid,
}

/// An in-progress packet. We currently do not know the use case.
#[derive(Debug, Clone)]
pub struct SimulationType {
    /// The simulation type selected.
    pub simulation_type: Simulation,
}

impl PacketType for SimulationType {
    fn write(&self, writer: &mut Writer) {
        writer.u8(self.simulation_type.to_u8().unwrap());
    }

    fn read(reader: &mut Reader) -> Self {
        Self { simulation_type: Simulation::from_u8(reader.u8()).unwrap() }
    }
}
