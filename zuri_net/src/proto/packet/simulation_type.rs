use zuri_net_derive::proto;

/// An in-progress packet. We currently do not know the use case.\
#[proto]
#[derive(Debug, Clone)]
pub struct SimulationType {
    /// The simulation type selected.
    pub simulation_type: Simulation,
}

#[proto(u8)]
#[derive(Debug, Clone)]
pub enum Simulation {
    Game,
    Editor,
    Test,
    Invalid,
}
