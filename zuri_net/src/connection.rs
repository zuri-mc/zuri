use bytes::Bytes;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use zuri_proto::io::{Reader, Writer};
use zuri_proto::packet::Packet;
use rust_raknet::{RaknetSocket, Reliability};
use rust_raknet::error::RaknetError;

use crate::encode::Encoder;
use crate::decode::Decoder;

pub struct Connection {
    socket: RaknetSocket,

    buffered_batch: Vec<Vec<u8>>,

    encoder: Encoder,
    decoder: Decoder,
}

impl Connection {
    pub fn new(socket: RaknetSocket) -> Self {
        Self {
            socket,

            buffered_batch: Vec::new(),

            encoder: Encoder::default(),
            decoder: Decoder::default(),
        }
    }

    pub async fn flush(&mut self) -> Result<(), ConnError> {
        let batch = self.encoder
            .encode(&mut self.buffered_batch)
            .map_err(|s| ConnError::EncodeError(s))?;

        self.buffered_batch.clear();

        Ok(self.socket.send(&batch, Reliability::ReliableOrdered).await?)
    }

    pub fn write_packet(&mut self, packet: &mut Packet) {
        let mut writer = Writer::new(0); // TODO: Shield ID
        packet.write(&mut writer);

        self.buffered_batch.push(writer.into());
    }

    pub async fn read_next_batch(&mut self) -> Result<Vec<Packet>, ConnError> {
        let encoded = self.socket.recv().await?;
        let batch = self.decoder
            .decode(&mut encoded.into())
            .map_err(|e| ConnError::DecodeError(e))?;

        let mut packets = Vec::with_capacity(batch.len());
        for buf in batch {
            let mut reader = Reader::from_buf(Bytes::from(buf), 0);
            packets.push(Packet::read(&mut reader));
        }

        Ok(packets)
    }
}

#[derive(Debug)]
pub enum ConnError {
    EncodeError(String),
    DecodeError(String),
    RakNetError(RaknetError),
}

impl Display for ConnError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnError::EncodeError(s) => f.write_str(&format!("Error encoding packet: {}", s)),
            ConnError::DecodeError(s) => f.write_str(&format!("Error decoding packet: {}", s)),
            ConnError::RakNetError(err) => f.write_str(&format!("RakNet error: {:?}", err)),
        }
    }
}

impl Error for ConnError {}

impl From<RaknetError> for ConnError {
    fn from(value: RaknetError) -> Self {
        Self::RakNetError(value)
    }
}
