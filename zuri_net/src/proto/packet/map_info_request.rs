use crate::proto::packet::PacketType;
use crate::proto::types::colour::RGBA;
use crate::proto::io::{Reader, Writer};

/// Sent by the client to request the server to deliver information of a certain map in the
/// inventory of the player. The server should respond with a ClientBoundMapItemData packet.
#[derive(Debug, Clone)]
pub struct MapInfoRequest {
    /// The unique identifier that represents the map that is requested over network. It remains
    /// consistent across sessions.
    pub map_id: i64,
    /// A list of pixels sent from the client to notify the server about the pixels that it isn't
    /// aware of.
    pub client_pixels: Vec<PixelRequest>,
}

impl PacketType for MapInfoRequest {
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

/// The request for the colour of a pixel in a MapInfoRequest packet.
#[derive(Debug, Clone)]
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
