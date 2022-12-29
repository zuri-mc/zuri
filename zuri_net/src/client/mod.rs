use std::net::SocketAddr;
use std::sync::Arc;

use async_trait::async_trait;
use oauth2::basic::BasicTokenResponse;
use rust_raknet::RaknetSocket;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::Mutex;
use crate::chan::{pk_chan, PkSender};

use crate::client::data::{ClientData, IdentityData};
use crate::client::login::{LoginData, LoginSequence};
use crate::connection::{Connection, ConnError, ExpectedPackets, Sequence};
use crate::proto::packet::Packet;

pub mod login;
mod auth;
pub mod data;

pub struct Client<H: Handler + Send + 'static> {
    conn: Arc<Connection>,
    handler: Arc<Mutex<H>>,
    seq_chan: Sender<(PkSender, Arc<ExpectedPackets>)>,

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
    ) -> Result<(Self, LoginData), ConnError> {
        let socket = RaknetSocket::connect_with_version(&ip, 11).await.expect("TODO: panic message"); // TODO: panic message

        let (send, recv) = channel(1);
        let (seq_send, seq_recv) = channel(1);
        let client = Self {
            conn: Arc::new(Connection::new(socket)),
            handler: Arc::new(Mutex::new(handler)),
            seq_chan: seq_send,

            client_data,
            identity_data: identity_data.unwrap_or(IdentityData {
                xuid: String::new(),
                identity: String::new(),
                display_name: String::new(),
                title_id: None,
            }), // TODO: Parse from live_token if present.
        };
        tokio::spawn(Self::read_loop(send, client.conn.clone(), seq_recv));
        tokio::spawn(Self::handle_loop(recv, client.handler.clone(), client.conn.clone()));

        let login_ret = client.exec_sequence(LoginSequence::new(
            &client.client_data,
            &client.identity_data,
            live_token,
            false,
        )).await?;
        Ok((client, login_ret))
    }

    pub async fn disconnect(&self) -> Result<(), String> {
        //self.conn.close()
        todo!()
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

    pub async fn exec_sequence<T>(&self, seq: impl Sequence<T>) -> T {
        let (send, recv) = pk_chan();
        let e = Arc::new(ExpectedPackets::default());
        self.seq_chan.send((send, e.clone())).await.expect("Could not send sequence to packet receiver");
        seq.execute(recv, self.conn.clone(), e).await
    }

    async fn read_loop(chan: Sender<Packet>, conn: Arc<Connection>, mut seq_recv: Receiver<(PkSender, Arc<ExpectedPackets>)>) {
        let mut seq_chan = None;
        let mut expecter = None;
        loop {
            let result = conn.read_next_packet().await;
            if seq_chan.is_none() {
                if let Ok((c, e)) = seq_recv.try_recv() {
                    seq_chan = Some(c);
                    expecter = Some(e);
                }
            }

            match result {
                Ok(pk) => {
                    if expecter.is_some() && expecter.as_ref().unwrap().expected(&pk).await {
                        let mut seq_done = false;
                        if let Some(c) = &mut seq_chan {
                            if !c.send(pk.clone()).await {
                                seq_done = true;
                            }
                            expecter.as_ref().unwrap().remove(&pk).await;
                        }
                        if seq_done {
                            seq_chan = None;
                            expecter = None;
                        }
                    } else {
                        // We can call expect here: the handler stops if the read loop stops.
                        chan.send(pk).await.expect("Could not send packet to handler");
                    }
                }
                Err(_) => {
                    return;
                }
            };
        }
    }

    async fn handle_loop<T: Handler + Send>(mut chan: Receiver<Packet>, handler: Arc<Mutex<T>>, conn: Arc<Connection>) {
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
    async fn handle_incoming(&mut self, _: Packet) -> Vec<Packet> { vec![] }
    async fn handle_outgoing(&mut self, _: &mut Packet) {}

    async fn handle_disconnect(&mut self, _: Option<String>) {}
}
