#![feature(default_free_fn)]
extern crate core;

pub mod io;
pub mod packet;
pub mod types;
mod r#macro;

pub const CURRENT_PROTOCOL: i32 = 560;
pub const CURRENT_VERSION: &str = "1.19.50";
