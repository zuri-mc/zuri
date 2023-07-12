pub mod ints;
pub mod io;
mod r#macro;
#[allow(clippy::all)] // todo: remove on cleanup.
pub mod packet;
#[allow(clippy::all)] // todo: remove on cleanup.
pub mod types;

pub const CURRENT_PROTOCOL: i32 = 594;
pub const CURRENT_VERSION: &str = "1.20.10";
