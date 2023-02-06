use crate::proto::io::{Readable, Reader, Writable, Writer};

#[derive(Debug, Clone)]
pub struct RGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Writable for RGBA {
    fn write(&self, writer: &mut Writer) {
        writer.u32((self.r as u32) | ((self.g as u32) << 8) | ((self.b as u32) << 16) | ((self.a as u32) << 24));
    }
}

impl Readable<RGBA> for RGBA {
    fn read(reader: &mut Reader) -> Self {
        let value = reader.u32();
        Self {
            r: value as u8,
            g: (value >> 8) as u8,
            b: (value >> 16) as u8,
            a: (value >> 24) as u8,
        }
    }
}

#[derive(Debug, Clone)]
pub struct VarRGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Writable for VarRGBA {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32((self.r as u32) | ((self.g as u32) << 8) | ((self.b as u32) << 16) | ((self.a as u32) << 24));
    }
}

impl Readable<VarRGBA> for VarRGBA {
    fn read(reader: &mut Reader) -> Self {
        let value = reader.var_u32();
        Self {
            r: value as u8,
            g: (value >> 8) as u8,
            b: (value >> 16) as u8,
            a: (value >> 24) as u8,
        }
    }
}
