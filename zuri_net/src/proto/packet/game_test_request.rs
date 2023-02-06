use zuri_net_derive::packet;
use crate::proto::ints::VarI32;

use crate::proto::io::BlockPos;

/// The purpose of this packet is currently unknown.
#[packet]
#[derive(Debug, Clone)]
pub struct GameTestRequest {
    /// The purpose of this field is currently unknown.
    pub max_tests_per_batch: VarI32,
    /// The amount of times the test will be run.
    pub repetitions: VarI32,
    /// The rotation of the test.
    pub rotation: GameTestRequestRotation,
    /// Indicates whether the test should immediately stop when an error is encountered.
    pub stop_on_error: bool,
    /// The position at which the test will be performed.
    pub position: BlockPos,
    /// The purpose of this field is currently unknown.
    pub tests_per_row: VarI32,
    /// The name of the test.
    pub name: String,
}

#[packet(u8)]
#[derive(Debug, Clone)]
pub enum GameTestRequestRotation {
    None,
    Rotate90,
    Rotate180,
    Rotate270,
    Rotate360,
}
