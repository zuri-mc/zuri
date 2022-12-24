use crate::packet::Packet;
use crate::types::colour::RGBA;
use crate::io::{Reader, Writer};

#[derive(Debug)]
pub struct MapInfoRequest {
    pub map_id: i64,
    pub client_pixels: Vec<PixelRequest>,
}

impl Packet for MapInfoRequest {
    fn write(&self, writer: &mut Writer) {
        writer.var_i64(self.map_id);
        writer.var_u32(self.client_pixels.len() as u32);
        self.client_pixels.iter().for_each(|pixel| pixel.write(writer));
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            map_id: reader.var_i64(),
            client_pixels: (0..reader.var_u32()).map(|_| PixelRequest::read(reader)).collect(),
        }
    }
}

#[derive(Debug)]
pub struct PixelRequest {
    colour: RGBA,
    index: u16,
}

impl PixelRequest {
    pub fn write(&self, writer: &mut Writer) {
        self.colour.write(writer);
        writer.u16(self.index);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            colour: RGBA::read(reader),
            index: reader.u16(),
        }
    }
}
