extern crate core;

pub mod chan;
pub mod client;
pub mod compression;
pub mod connection;
pub mod encode;
pub mod encryption;
pub mod proto;
pub mod server;

#[cfg(test)]
mod tests {
    use crate::proto::ints::VarU32;
    use crate::proto::io::{Readable, Reader, Writable, Writer};
    use bytes::Bytes;
    use zuri_net_derive::proto;

    #[proto]
    #[derive(PartialEq, Debug)]
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

    #[proto]
    struct UnitPacket;

    #[proto(u32)] // the type contained in the brackets is the default type to write this enum with
    #[derive(PartialEq, Debug)]
    enum EnumPacket {
        Variant1,
        Variant2,
        Variant3,
        Variant4 = 83,
        Variant5,
    }

    #[proto(u32)]
    enum DataEnumPacket {
        Variant1(Data1),
        Variant2(Data2),
        Variant3,
        Variant4(Data1, Data2),
    }

    #[proto]
    struct Data1;
    #[proto]
    struct Data2;

    #[test]
    fn read_write_test() {
        let mut writer = Writer::new(0);
        let pk_from = TestPacket {
            test: "Example string".to_string(),
            test2: 20,
            some_field: true,
            test_vec: vec!["beep".to_string(), "boop".to_string(), "".to_string()],
            test_vec2: vec![],
            my_enum: EnumPacket::Variant4,
        };
        pk_from.write(&mut writer);

        let mut writer2 = Writer::new(0);
        writer2.string("Example string");
        writer2.i64(20);
        writer2.u32(3);
        writer2.u32(0);
        writer2.bool(true);
        writer2.string("beep");
        writer2.string("boop");
        writer2.string("");
        writer2.var_u32(83);

        let bytes: Bytes = writer.into();
        let bytes2: Bytes = writer2.into();
        assert_eq!(bytes, bytes2);

        let mut reader = Reader::from_buf(bytes, 0);
        let pk_to = TestPacket::read(&mut reader);
        assert_eq!(pk_from, pk_to);
    }
}
