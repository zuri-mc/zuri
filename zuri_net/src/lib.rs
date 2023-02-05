#![feature(associated_type_bounds)]

extern crate core;

pub mod client;
pub mod compression;
pub mod connection;
pub mod encode;
pub mod encryption;
pub mod proto;
pub mod chan;

#[cfg(test)]
mod tests {
    use zuri_net_derive::packet;
    use crate::proto::ints::VarU32;
    use crate::proto::io::{EnumReadable, EnumWritable, Writable, Writer};
    use crate::proto::io::Readable;

    #[packet]
    struct TestPacket {
        pub test: String,
        pub test2: i64,
        #[size_for(test_vec)]
        __: u32,
        #[size_for(test_vec2)]
        __: u32,
        pub some_field: bool,
        pub test_vec: Vec<String>,
        pub test_vec2: Vec<String>,
        #[enum_header(VarU32)]
        pub my_enum: EnumPacket,
    }

    #[packet]
    struct UnitPacket;

    #[packet(u32)] // the type contained in the brackets is the default type to write this enum with
    enum EnumPacket {
        Variant1,
        Variant2,
        Variant3,
        Variant4 = 83,
        Variant5,
    }

    fn temp_test_enums() {
        let pk = EnumPacket::Variant1;
        let mut w = Writer::new(0);
        <EnumPacket as EnumWritable<VarU32>>::write(&pk, &mut w);
    }
}
