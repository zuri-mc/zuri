use std::collections::VecDeque;
use bytes::Bytes;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use async_trait::async_trait;
use p384::ecdsa::SigningKey;
use zuri_proto::io::{Reader, Writer};
use zuri_proto::packet::Packet;
use rust_raknet::{RaknetSocket, Reliability};
use rust_raknet::error::RaknetError;
use tokio::sync::Mutex;
use crate::compression::Compression;

use crate::encode::Encoder;
use crate::encryption::Encryption;

pub struct Connection {
    socket: RaknetSocket,

    buffered_batch: Vec<Vec<u8>>,
    queued_packets: VecDeque<Packet>,

    signing_key: SigningKey,

    encoder: Encoder,
}

impl Connection {
    pub fn new(socket: RaknetSocket) -> Self {
        Self {
            socket,

            buffered_batch: Vec::new(),
            queued_packets: VecDeque::new(),

            signing_key: SigningKey::random(&mut rand::thread_rng()),

            encoder: Encoder::default(),
        }
    }

    pub fn signing_key(&self) -> &SigningKey {
        &self.signing_key
    }

    pub fn set_compression(&mut self, compression: Compression) {
        self.encoder.set_compression(compression);
    }

    pub fn set_encryption(&mut self, encryption: Encryption) {
        self.encoder.set_encryption(encryption);
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

    pub async fn read_next_packet(&mut self) -> Result<Packet, ConnError> {
        loop {
            if let Some(packet) = self.queued_packets.pop_front() {
                return Ok(packet);
            }
            self.queued_packets = self.read_next_batch().await?.into();
        }
    }

    async fn read_next_batch(&mut self) -> Result<Vec<Packet>, ConnError> {
        let encoded = self.socket.recv().await?;
        let batch = self.encoder
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

#[async_trait]
pub trait Sequence<E> {
    async fn execute(self, conn: &Mutex<Connection>) -> Result<(), E>;
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
