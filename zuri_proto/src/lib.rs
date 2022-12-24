#![feature(default_free_fn)]
extern crate core;

mod io;
mod r#macro;
mod packet;
mod types;

pub const CURRENT_PROTOCOL: u32 = 560;
pub const CURRENT_VERSION: &'static str = "1.19.50";
