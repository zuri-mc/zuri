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

    #[packet]
    struct TestPacket {
        pub test: String,
        pub test2: i64,
        #[len_for(test_vec)]
        __: u32,
        #[len_for(test_vec2)]
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

    #[packet(u32)]
    enum DataEnumPacket {
        Variant1(Data1),
        Variant2(Data2),
        Variant3,
        Variant4(Data1, Data2),
    }

    #[packet]
    struct Data1;
    #[packet]
    struct Data2;

}
