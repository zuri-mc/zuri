use std::borrow::BorrowMut;
use std::net::{SocketAddr};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use rust_raknet::RaknetSocket;
use zuri_proto::packet::Packet;
use zuri_proto::packet::request_network_settings::RequestNetworkSettings;

use crate::data::{ClientData, IdentityData};
use crate::connection::{Connection, ConnError};

pub struct Client<H: Handler> {
    stage: LoginStage,

    conn: Arc<Connection>,
    handler: Arc<Mutex<H>>,

    client_data: ClientData,
    identity_data: IdentityData,
}

pub enum LoginStage {
    NetworkSettings,
    ServerToClientHandshake,
    PlayStatusLoginSuccess,
    ResourcePacksInfo,
    ResourcePackStack,
    StartGame,
    CreativeContent,
    BiomeDefinitions,
    LevelChunk,
    PlayStatus,
    Success,
}

impl<H: Handler> Client<H> {
    pub async fn connect(
        ip: impl Into<SocketAddr>,
        client_data: ClientData,
        identity_data: IdentityData,
        handler: H,
    ) -> Result<Self, String> {
        let socket = RaknetSocket::connect(&ip.into()).await.expect("TODO: panic message"); // TODO: panic message

        let (send, recv) = channel();
        let mut client = Self {
            stage: LoginStage::NetworkSettings,

            conn: Arc::new(Connection::new(socket)),
            handler: Arc::new(Mutex::new(handler)),

            client_data,
            identity_data,
        };
        tokio::spawn(Self::read_loop(send, client.conn.clone()));
        tokio::spawn(Self::handle_loop(recv, client.handler.clone()));

        client.write_packet(&mut Packet::RequestNetworkSettings(RequestNetworkSettings {
            client_protocol: zuri_proto::CURRENT_PROTOCOL,
        })).await.expect("TODO: panic message"); // TODO: Panic message
        Ok(client)
    }

    pub async fn disconnect(&mut self) -> Result<(), String> {
        //self.conn.close()
        todo!()
    }

    pub async fn write_packet(&mut self, packet: &mut Packet) -> Result<(), ConnError> {
        let mu = self.handler.lock().unwrap();
        mu.handle_outgoing(packet);
        drop(mu);

        self.conn.write_packet(packet);
        Ok(())
    }

    async fn read_loop(chan: Sender<Packet>, mut conn: Arc<Connection>) {
        loop {
            match conn.borrow_mut().read_next_batch().await {
                Ok(pks) => {
                    for pk in pks {
                        // We can call expect here: the handler stops if the read loop stops.
                        chan.send(pk).expect("Could not send packet to handler");
                    }
                }
                Err(_) => {
                    return;
                }
            };
        }
    }

    async fn handle_loop<T: Handler>(chan: Receiver<Packet>, handler: Arc<Mutex<T>>) {
        loop {
            if let Ok(pk) = chan.recv() {
                handler.lock().unwrap().handle_incoming(pk);
            } else {
                handler.lock().unwrap().handle_disconnect(None); // todo: reason
                return;
            }
        }
    }
}

/// Handles events such as incoming packets from the connection.
pub trait Handler {
    fn handle_incoming(&self, pk: Packet) {}
    fn handle_outgoing(&self, pk: &mut Packet) {}

    fn handle_disconnect(&self, reason: Option<String>) {}
}

// tests
#[cfg(test)]
mod tests {
    use uuid::Uuid;
    use super::*;

    #[test]
    fn connect_test() {
        let mut rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let mut client = Client::connect(
                "127.0.0.1:19132",
                ClientData::default(),
                IdentityData {
                    display_name: "Zuri".into(),
                    identity: Uuid::new_v4().to_string(),
                    title_id: None,
                    xuid: None,
                },
                TestHandler,
            ).await.unwrap();
        });
    }

    struct TestHandler;

    impl Handler for TestHandler {
        fn handle_incoming(&self, pk: Packet) {
            println!("Incoming packet: {:?}", pk);
        }

        fn handle_outgoing(&self, pk: &mut Packet) {
            println!("Outgoing packet: {:?}", pk);
        }
    }
}
