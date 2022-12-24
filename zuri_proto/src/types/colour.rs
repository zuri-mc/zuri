use crate::io::{Reader, Writer};

#[derive(Debug)]
pub struct RGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl RGBA {
    pub fn write(&self, writer: &mut Writer) {
        writer.u32((self.r as u32) | ((self.g as u32) << 8) | ((self.b as u32) << 16) | ((self.a as u32) << 24));
    }

    pub fn write_var(&self, writer: &mut Writer) {
        writer.var_u32((self.r as u32) | ((self.g as u32) << 8) | ((self.b as u32) << 16) | ((self.a as u32) << 24));
    }

    pub fn read(reader: &mut Reader) -> Self {
        let value = reader.u32();
        Self {
            r: value as u8,
            g: (value >> 8) as u8,
            b: (value >> 16) as u8,
            a: (value >> 24) as u8,
        }
    }

    pub fn read_var(reader: &mut Reader) -> Self {
        let value = reader.var_u32();
        Self {
            r: value as u8,
            g: (value >> 8) as u8,
            b: (value >> 16) as u8,
            a: (value >> 24) as u8,
        }
    }
}
