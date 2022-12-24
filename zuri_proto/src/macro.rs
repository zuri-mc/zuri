#[macro_export]
macro_rules! encodable_enum {
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
                return match reader.var_u32() & 0x3FF {
                    $($discrim => $name::$elem($elem::read(reader)),)+
                    _ => panic!("unknown enum variant"),
                }
            }

            $vis fn write(&self, writer: &mut Writer) {
                match self {
                    $($name::$elem(pk) => {
                        writer.var_u32($discrim);
                        pk.write(writer);
                    },)+
                    _ => panic!("unknown enum variant"),
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