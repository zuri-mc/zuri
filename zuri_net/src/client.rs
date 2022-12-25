use std::net::SocketAddr;
use std::sync::{Arc};
use rust_raknet::RaknetSocket;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::Mutex;
use zuri_proto::packet::Packet;
use zuri_proto::packet::request_network_settings::RequestNetworkSettings;

use crate::data::{ClientData, IdentityData};
use crate::connection::{Connection, ConnError};

pub struct Client<H: Handler + Send + 'static> {
    stage: LoginStage,

    conn: Arc<Mutex<Connection>>,
    handler: Arc<Mutex<H>>,

    client_data: ClientData,
    identity_data: IdentityData,
}

impl<H: Handler + Send + 'static> Client<H> {
    pub async fn connect(
        ip: &SocketAddr,
        client_data: ClientData,
        identity_data: IdentityData,
        handler: H,
    ) -> Result<Self, String> {
        let socket = RaknetSocket::connect_with_version(ip, 11).await.expect("TODO: panic message"); // TODO: panic message

        let (send, recv) = channel(1);
        let mut client = Self {
            stage: LoginStage::NetworkSettings,

            conn: Arc::new(Mutex::new(Connection::new(socket))),
            handler: Arc::new(Mutex::new(handler)),

            client_data,
            identity_data,
        };
        tokio::spawn(Self::read_loop(send, client.conn.clone()));
        tokio::spawn(Self::handle_loop(recv, client.handler.clone(), client.conn.clone()));

        client.write_packet(&mut Packet::RequestNetworkSettings(RequestNetworkSettings {
            client_protocol: zuri_proto::CURRENT_PROTOCOL,
        })).await.expect("TODO: panic message"); // TODO: Panic message
        client.flush().await.expect("TODO: panic message");

        Ok(client)
    }

    pub async fn disconnect(&mut self) -> Result<(), String> {
        //self.conn.close()
        todo!()
    }

    pub async fn write_packet(&mut self, packet: &mut Packet) -> Result<(), ConnError> {
        let mut mu = self.handler.lock().await;
        mu.handle_outgoing(packet);
        drop(mu);

        self.conn.lock().await.write_packet(packet);
        Ok(())
    }

    pub async fn flush(&mut self) -> Result<(), ConnError> {
        self.conn.lock().await.flush().await
    }

    async fn read_loop(chan: Sender<Packet>, conn: Arc<Mutex<Connection>>) {
        loop {
            let pks = conn.lock().await.read_next_batch().await;
            //let pks = b.await;
            //let pks = mu.read_next_batch().await;
            //drop(mu);
            match pks {
                Ok(pks) => {
                    for pk in pks {
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

    async fn handle_loop<T: Handler>(mut chan: Receiver<Packet>, handler: Arc<Mutex<T>>, conn: Arc<Mutex<Connection>>) {
        loop {
            if let Some(pk) = chan.recv().await {
                let mut response = handler.lock().await.handle_incoming(pk);

                let mut mu = conn.lock().await;
                for pk in &mut response {
                    mu.write_packet(pk);
                }
                mu.flush();
            } else {
                handler.lock().await.handle_disconnect(None); // todo: reason
                return;
            }
        }
    }
}

/// Handles events such as incoming packets from the connection.
pub trait Handler {
    fn handle_incoming(&mut self, pk: Packet) -> Vec<Packet> { vec![] }
    fn handle_outgoing(&mut self, pk: &mut Packet) {}

    fn handle_disconnect(&mut self, reason: Option<String>) {}
}

#[derive(Copy, Clone, PartialEq)]
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

impl LoginStage {
    pub fn advance(&mut self) -> Result<(), ()> {
        match self {
            LoginStage::NetworkSettings => *self = LoginStage::ServerToClientHandshake,
            LoginStage::ServerToClientHandshake => *self = LoginStage::PlayStatusLoginSuccess,
            LoginStage::PlayStatusLoginSuccess => *self = LoginStage::ResourcePacksInfo,
            LoginStage::ResourcePacksInfo => *self = LoginStage::ResourcePackStack,
            LoginStage::ResourcePackStack => *self = LoginStage::StartGame,
            LoginStage::StartGame => *self = LoginStage::CreativeContent,
            LoginStage::CreativeContent => *self = LoginStage::BiomeDefinitions,
            LoginStage::BiomeDefinitions => *self = LoginStage::LevelChunk,
            LoginStage::LevelChunk => *self = LoginStage::PlayStatus,
            LoginStage::PlayStatus => *self = LoginStage::Success,
            LoginStage::Success => return Err(()),
        };
        Ok(())
    }
}

// tests
#[cfg(test)]
mod tests {
    use std::time::Duration;
    use tokio::time::sleep;
    use uuid::Uuid;
    use super::*;

    #[tokio::test]
    async fn connect_test() {
        let mut client = Client::connect(
            &"127.0.0.1:19132".parse().unwrap(),
            //&"172.10.117.138:19132".parse().unwrap(),
            ClientData::default(),
            IdentityData {
                display_name: "Zuri".into(),
                identity: Uuid::new_v4().to_string(),
                title_id: None,
                xuid: None,
            },
            TestHandler,
        ).await.unwrap();
        sleep(Duration::from_secs(3)).await;
    }

    struct TestHandler;

    impl Handler for TestHandler {
        fn handle_incoming(&mut self, pk: Packet) -> Vec<Packet> {
            println!("Incoming packet: {:?}", pk);
            match pk {
                Packet::NetworkSettings(pk) => {


                }
                _ => todo!()
            }
            vec![]
        }

        fn handle_outgoing(&mut self, pk: &mut Packet) {
            println!("Outgoing packet: {:?}", pk);
        }
    }
}
