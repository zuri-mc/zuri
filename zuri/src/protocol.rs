use crate::io::{Reader, Writer};

macro_rules! packets {
    (
            $(#[$attr:meta])*
            $vis:vis enum $name:ident {
                $($elem:ident = $discrim:literal$(,)?)+
            }
    ) => {
        $(#[$attr])*
        #[repr(u32)]
        $vis enum $name {
            $($elem($elem) = $discrim,)+
        }

        impl $name {
            $vis fn read(reader: &mut Reader) -> Self {
                return match reader.read_var_u32() & 0x3FF {
                    $($discrim => $name::$elem($elem::read(reader)),)+
                    _ => panic!("Unknown packet type"),
                }
            }

            $vis fn write(&self, writer: &mut Writer) {
                match self {
                    $($name::$elem(pk) => {
                        writer.write_var_u32($discrim);
                        pk.write(writer);
                    },)+
                    _ => panic!("Unknown packet type"),
                }
            }
        }

        /// Allow the packets to be converted to the enum with Into.
        $(impl Into<$name> for $elem {
            fn into(self) -> $name {
                $name::$elem(self)
            }
        })+
    };
}

packets!(
    pub enum Packets {
        RequestNetworkSettings = 0,
        NetworkSettings = 1,
    }
);

struct RequestNetworkSettings {
    client_protocol: i32,
}

trait Packet {
    fn write(&self, writer: &mut Writer);
    fn read(reader: &mut Reader) -> Self;
}

impl Packet for RequestNetworkSettings {
    fn write(&self, writer: &mut Writer) {
        writer.write_be32(self.client_protocol);
    }

    fn read(reader: &mut Reader) -> Self {
        return RequestNetworkSettings { client_protocol: reader.read_be32() };
    }
}

struct NetworkSettings {
    compression_threshold: u16,
    compression_algorithm: u16,

    client_throttle: bool,
    client_throttle_threshold: u8,
    client_throttle_scalar: f32,
}

impl Packet for NetworkSettings {
    fn write(&self, writer: &mut Writer) {
        writer.write_u16(self.compression_threshold);
        writer.write_u16(self.compression_algorithm);
        writer.write_bool(self.client_throttle);
        writer.write_u8(self.client_throttle_threshold);
        writer.write_f32(self.client_throttle_scalar);
    }

    fn read(reader: &mut Reader) -> Self {
        return NetworkSettings {
            compression_threshold: reader.read_u16(),
            compression_algorithm: reader.read_u16(),
            client_throttle: true, // todo
            client_throttle_threshold: reader.read_u8(),
            client_throttle_scalar: 1., // todo
        };
    }
}
