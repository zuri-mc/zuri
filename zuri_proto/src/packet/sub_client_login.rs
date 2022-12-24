#[derive(Debug)]
pub struct SubClientLogin {
    pub connection_request: Bytes,
}

impl Packet for SubClientLogin {
    fn write(&self, writer: &mut Writer) {
        writer.byte_slice(&self.connection_request);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            connection_request: reader.byte_slice(),
        }
    }
}
