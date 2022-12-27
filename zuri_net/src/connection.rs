use std::collections::VecDeque;
use bytes::Bytes;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::sync::Arc;
use async_trait::async_trait;
use p384::ecdsa::SigningKey;
use crate::proto::io::{Reader, Writer};
use crate::proto::packet::Packet;
use rust_raknet::{RaknetSocket, Reliability};
use rust_raknet::error::RaknetError;
use tokio::sync::Mutex;
use crate::compression::Compression;

use crate::encode::Encoder;
use crate::encryption::Encryption;

pub struct Connection {
    socket: RaknetSocket,

    buffered_batch: Mutex<Vec<Vec<u8>>>,
    queued_packets: Mutex<VecDeque<Packet>>,

    signing_key: SigningKey,

    encoder: Mutex<Encoder>,
}

impl Connection {
    pub fn new(socket: RaknetSocket) -> Self {
        Self {
            socket,

            buffered_batch: Mutex::new(Vec::new()),
            queued_packets: Mutex::new(VecDeque::new()),

            signing_key: SigningKey::random(&mut rand::thread_rng()),

            encoder: Mutex::new(Encoder::default()),
        }
    }

    pub fn signing_key(&self) -> &SigningKey {
        &self.signing_key
    }

    pub async fn set_compression(&self, compression: Compression) {
        self.encoder.lock().await.set_compression(compression);
    }

    pub async fn set_encryption(&self, encryption: Encryption) {
        self.encoder.lock().await.set_encryption(encryption);
    }

    pub async fn flush(&self) -> Result<(), ConnError> {
        let mut batch_mu = self.buffered_batch.lock().await;
        let batch = self.encoder.lock().await
            .encode(&mut *batch_mu)
            .map_err(|s| ConnError::EncodeError(s))?;
        batch_mu.clear();
        drop(batch_mu);

        Ok(self.socket.send(&batch, Reliability::ReliableOrdered).await?)
    }

    pub async fn write_packet(&self, packet: &mut Packet) {
        let mut writer = Writer::new(0); // TODO: Shield ID
        packet.write(&mut writer);

        self.buffered_batch.lock().await.push(writer.into());
    }

    pub async fn read_next_packet(&self) -> Result<Packet, ConnError> {
        loop {
            let mut queue = self.queued_packets.lock().await;
            if let Some(packet) = queue.pop_front() {
                return Ok(packet);
            }
            *queue = self.read_next_batch().await?.into();
        }
    }

    async fn read_next_batch(&self) -> Result<Vec<Packet>, ConnError> {
        let encoded = self.socket.recv().await?;
        let batch = self.encoder.lock().await
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
    async fn execute(self, conn: Arc<Connection>) -> Result<(), E>;
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
