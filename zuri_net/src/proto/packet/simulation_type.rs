use zuri_net_derive::packet;

/// An in-progress packet. We currently do not know the use case.\
#[packet]
#[derive(Debug, Clone)]
pub struct SimulationType {
    /// The simulation type selected.
    pub simulation_type: Simulation,
}

#[packet(u8)]
#[derive(Debug, Clone)]
pub enum Simulation {
    Game,
    Editor,
    Test,
    Invalid,
}
