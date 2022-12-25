use bytes::Bytes;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Write};
use zuri_proto::io::{Reader, Writer};
use zuri_proto::packet::Packet;
use rust_raknet::{RaknetSocket, Reliability};
use rust_raknet::error::RaknetError;

use crate::encode::Encoder;
use crate::decode::Decoder;

pub struct Connection {
    socket: RaknetSocket,

    buffered_batch: Vec<Bytes>,

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
        let batch = self.encoder.encode(&mut self.buffered_batch)?;
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
        let batch = self.decoder.decode(&mut encoded.into())?;

        let mut packets = Vec::with_capacity(batch.len());
        for buf in batch {
            let mut reader = Reader::from_buf(buf, 0);
            packets.push(Packet::read(&mut reader));
        }

        Ok(packets)
    }
}

#[derive(Debug)]
pub enum ConnError {
    ConnClosed,
    DecodeError(String),
    RakNetError(RaknetError),
}

impl Display for ConnError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnError::ConnClosed => f.write_str("Connection is closed"),
            ConnError::DecodeError(s) => f.write_str(&format!("Error while decoding: {}", s)),
            ConnError::RakNetError(err) => f.write_str("RakNet error: ") + err.fmt(),
        }
    }
}

impl Error for ConnError {}

impl From<RaknetError> for ConnError {
    fn from(value: RaknetError) -> Self {
        Self::RakNetError(value)
    }
}