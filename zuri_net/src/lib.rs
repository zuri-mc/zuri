extern crate core;

use zuri_net_derive::packet;

pub mod client;
pub mod compression;
pub mod connection;
pub mod encode;
pub mod encryption;
pub mod proto;
pub mod chan;

#[packet]
struct Test {
    pub test: String,
    pub test2: i64,
    #[size_for(test_vec)]
    __: u32,
    #[size_for(test_vec2)]
    __: u32,
    pub some_field: bool,
    pub test_vec: Vec<String>,
    pub test_vec2: Vec<String>,
}
