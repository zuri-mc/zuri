use std::net::SocketAddr;
use std::sync::Arc;

use async_trait::async_trait;
use oauth2::basic::BasicTokenResponse;
use rust_raknet::RaknetSocket;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::Mutex;

use crate::client::data::{ClientData, IdentityData};
use crate::client::login::LoginSequence;
use crate::connection::{ConnError, Connection};
use crate::proto::packet::Packet;

mod auth;
pub mod data;
pub mod login;

pub struct Client<H: Handler + Send + 'static> {
    conn: Arc<Connection>,
    handler: Arc<Mutex<H>>,

    client_data: ClientData,
    identity_data: IdentityData,
}

impl<H: Handler + Send + 'static> Client<H> {
    pub async fn connect(
        ip: SocketAddr,
        client_data: ClientData,
        identity_data: Option<IdentityData>,
        live_token: Option<BasicTokenResponse>,
        handler: H,
    ) -> Result<Self, ConnError> {
        let guaranteed_identity_data = identity_data.unwrap_or(IdentityData {
            xuid: String::new(),
            identity: String::new(),
            display_name: String::new(),
            title_id: None,
        }); // TODO: Parse from live_token if present.

        let mut guaranteed_client_data = client_data.clone();
        guaranteed_client_data.server_address = ip.to_string();
        guaranteed_client_data.third_party_name = guaranteed_identity_data.display_name.clone();

        let socket = RaknetSocket::connect_with_version(&ip, 11)
            .await
            .expect("TODO: panic message"); // TODO: panic message

        let (send, recv) = channel(1);
        let client = Self {
            conn: Arc::new(Connection::new(socket)),
            handler: Arc::new(Mutex::new(handler)),

            client_data: guaranteed_client_data,
            identity_data: guaranteed_identity_data,
        };
        tokio::spawn(Self::read_loop(send, client.conn.clone()));
        tokio::spawn(Self::handle_loop(
            recv,
            client.handler.clone(),
            client.conn.clone(),
        ));

        client
            .conn
            .exec_sequence(LoginSequence::new(
                &client.client_data,
                &client.identity_data,
                live_token,
                false,
            ))
            .await?;
        Ok(client)
    }

    pub async fn disconnect(&self) {
        let _ = self.conn.close().await.map_err(|_| unreachable!());
    }

    pub async fn write_packet(&self, packet: &mut Packet) -> Result<(), ConnError> {
        let mut mu = self.handler.lock().await;
        mu.handle_outgoing(packet).await;
        drop(mu);

        self.conn.write_packet(packet).await;
        Ok(())
    }

    pub async fn flush(&self) -> Result<(), ConnError> {
        self.conn.flush().await
    }

    async fn read_loop(chan: Sender<Packet>, conn: Arc<Connection>) {
        loop {
            let result = conn.read_next_packet().await;

            match result {
                Ok(pk) => {
                    // We can call expect here: the handler stops if the read loop stops.
                    chan.send(pk)
                        .await
                        .expect("Could not send packet to handler")
                }
                Err(_) => {
                    return;
                }
            };
        }
    }

    async fn handle_loop<T: Handler + Send>(
        mut chan: Receiver<Packet>,
        handler: Arc<Mutex<T>>,
        conn: Arc<Connection>,
    ) {
        loop {
            if let Some(pk) = chan.recv().await {
                let mut response = handler.lock().await.handle_incoming(pk).await;

                for pk in &mut response {
                    conn.write_packet(pk).await;
                }
                conn.flush().await.unwrap();
            } else {
                handler.lock().await.handle_disconnect(None).await; // todo: reason
                return;
            }
        }
    }
}

/// Handles events such as incoming packets from the connection.
#[async_trait]
pub trait Handler {
    async fn handle_incoming(&mut self, _: Packet) -> Vec<Packet> {
        vec![]
    }
    async fn handle_outgoing(&mut self, _: &mut Packet) {}

    async fn handle_disconnect(&mut self, _: Option<String>) {}
}
