use std::borrow::BorrowMut;
use std::future::Future;
use std::net::{SocketAddr};
use std::process::id;
use std::sync::{Arc};
use rust_raknet::RaknetSocket;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::Mutex;
use zuri_proto::packet::client_to_server_handshake::ClientToServerHandshake;
use zuri_proto::packet::network_settings::NetworkSettings;
use zuri_proto::packet::Packet;
use zuri_proto::packet::request_network_settings::RequestNetworkSettings;

use crate::data::{ClientData, IdentityData};
use crate::connection::{Connection, ConnError};

type StdClient<H: Handler> = Client<ClientLogin<H>>;

pub struct Client<H: Handler> {
    stage: LoginStage,

    conn: Arc<Mutex<Connection>>, // todo: dont require connection to be in a mutex
    handler: Arc<Mutex<H>>,

    client_data: ClientData,
    identity_data: IdentityData,
}

impl<H: Handler> Client<H> {
    pub async fn connect(
        ip: impl Into<SocketAddr>,
        client_data: ClientData,
        identity_data: IdentityData,
        handler: H,
    ) -> Result<StdClient<H>, String> {
        return match Client::connect_without_login(ip, client_data, identity_data, ClientLogin::new(handler)).await {
            Ok(mut client) => {
                client.write_packet(&mut Packet::RequestNetworkSettings(RequestNetworkSettings {
                    client_protocol: zuri_proto::CURRENT_PROTOCOL,
                })).await.expect("TODO: panic message"); // TODO: Panic message

                Ok(client)
            }
            Err(e) => {
                Err(e)
            }
        }
    }

    pub async fn connect_without_login(
        ip: impl Into<SocketAddr>,
        client_data: ClientData,
        identity_data: IdentityData,
        handler: H,
    ) -> Result<Self, String> {
        let socket = RaknetSocket::connect(&ip.into()).await.expect("TODO: panic message"); // TODO: panic message

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
        Ok(client)
    }

    pub async fn disconnect(&mut self) -> Result<(), String> {
        //self.conn.close()
        todo!()
    }

    pub async fn write_packet(&mut self, packet: &mut Packet) -> Result<(), ConnError> {
        let mut mu = self.handler.lock().unwrap();
        mu.handle_outgoing(packet);
        drop(mu);

        self.conn.write_packet(packet);
        Ok(())
    }

    async fn read_loop(chan: Sender<Packet>, mut conn: Arc<Mutex<Connection>>) {
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

    async fn handle_loop<T: Handler>(mut chan: Receiver<Packet>, handler: Arc<Mutex<T>>, mut conn: Arc<Mutex<Connection>>) {
        loop {
            if let Ok(pk) = chan.recv().await {
                match handler.lock().unwrap().handle_incoming(pk) {
                    Response::Send(mut pks) => {
                        let mut mu = conn.lock().unwrap();
                        for pk in &mut pks {
                            conn.write_packet(pk);
                        }
                        drop(mu);
                    }
                    Response::SendImmediate(mut pks) => {
                        let mut mu = conn.lock().unwrap();
                        for pk in &mut pks {
                            // todo: send immediate
                            conn.write_packet(pk);
                        }
                        drop(mu);
                    },
                    Response::None => continue,
                }
            } else {
                handler.lock().unwrap().handle_disconnect(None); // todo: reason
                return;
            }
        }
    }
}

/// Returned by the handler on handle_incoming. Sends a message to the connect to send one or
/// multiple (or no) packets back to the server. They can be sent either immediately or be buffered.
pub enum Response {
    Send(Vec<Packet>),
    SendImmediate(Vec<Packet>),
    None,
}

/// Handles events such as incoming packets from the connection.
pub trait Handler {
    fn handle_incoming(&mut self, pk: Packet) -> Response { Response::None }
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

pub struct ClientLogin<H: Handler> {
    stage: LoginStage,

    inner_handler: H,
}

impl<H: Handler> ClientLogin<H> {
    pub fn new(handler: H) -> Self {
        Self {
            stage: LoginStage::NetworkSettings,
            inner_handler: handler,
        }
    }
}

impl<H: Handler> Handler for ClientLogin<H> {
    fn handle_incoming(&mut self, pk: Packet) -> Response {
        match self.stage {
            LoginStage::NetworkSettings => {
                let net_set: Result<NetworkSettings, ()> = pk.into();
                if net_set.is_err() {
                    todo!();
                }
                self.stage.advance().unwrap();
                Response::SendImmediate(vec![ClientToServerHandshake.into()])
            }
            LoginStage::ServerToClientHandshake => todo!(),
            LoginStage::PlayStatusLoginSuccess => todo!(),
            LoginStage::ResourcePacksInfo => todo!(),
            LoginStage::ResourcePackStack => todo!(),
            LoginStage::StartGame => todo!(),
            LoginStage::CreativeContent => todo!(),
            LoginStage::BiomeDefinitions => todo!(),
            LoginStage::LevelChunk => todo!(),
            LoginStage::PlayStatus => todo!(),
            // When the login sequence was successful, pass the packets along to the actual handler.
            LoginStage::Success => self.inner_handler.handle_incoming(pk),
        }
    }

    fn handle_outgoing(&mut self, pk: &mut Packet) {
        if self.stage != LoginStage::Success {
            // Abstract login sequence from the inner handler.
            return;
        }
        self.inner_handler.handle_outgoing(pk);
    }

    fn handle_disconnect(&mut self, reason: Option<String>) {
        if self.stage != LoginStage::Success {
            todo!(); // login error
        }
        self.inner_handler.handle_disconnect(reason);
    }
}

// tests
#[cfg(test)]
mod tests {
    use uuid::Uuid;
    use super::*;

    #[test] // todo: use tokio::test here
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
        fn handle_incoming(&mut self, pk: Packet) -> Response {
            println!("Incoming packet: {:?}", pk);
            Response::None
        }

        fn handle_outgoing(&mut self, pk: &mut Packet) {
            println!("Outgoing packet: {:?}", pk);
        }
    }
}
