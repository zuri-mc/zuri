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
                }
            }
        }

        /// Allow the variants to be converted to the enum with Into.
        $(impl From<$elem> for $name {
            fn from(e: $elem) -> $name {
                $name::$elem(e)
            }
        })+

        /// Allow the enum to converted into a variant itself if it matches the variant.
        $(impl TryFrom<$name> for $elem {
            type Error = (); // todo: some kind of error here?

            fn try_from(value: $name) -> Result<Self, Self::Error> {
                match value {
                    $name::$elem(e) => Ok(e),
                    _ => Err(()),
                }
            }
        })+
    };
}
