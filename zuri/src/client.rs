use std::env;
use std::net::ToSocketAddrs;
use std::sync::Arc;
use async_trait::async_trait;
use bevy::app::AppExit;

use bevy::prelude::*;
use futures_lite::future;
use oauth2::devicecode::StandardDeviceAuthorizationResponse;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::mpsc::error::TryRecvError;
use tokio::task::JoinHandle;
use uuid::Uuid;
use zuri_net::client::{Client, Handler};
use zuri_net::client::data::{ClientData, IdentityData};
use zuri_net::client::login::LoginData;
use zuri_net::connection::ConnError;
use zuri_net::proto::packet::Packet;
use zuri_xbox::live;

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app
            // Special case for the event for the sending of packets. Initializing the resource
            // directly causes it to never be cleared automatically.
            .init_resource::<Events<Packet>>()
            .add_startup_system(init_client)
            .add_system(client_connection_system)
            .add_system(receive_packets)
            .add_system(send_packets);
    }
}

pub struct ClientWaiter {
    task: JoinHandle<Result<(Client<PacketHandler>, LoginData), ConnError>>,
}

fn init_client(world: &mut World) {
    let address = env::var("zuri_ip").unwrap_or("127.0.0.1:19132".into());

    let mut identity_data = None;
    let mut live_token = None;
    if env::var("xbox").unwrap_or("false".into()).to_lowercase() == "true" {
        live_token = Some(live::read_or_obtain_token(
            "token.tok".into(),
            |details: &StandardDeviceAuthorizationResponse| {
                println!(
                    "Authenticate at {} using the code: {}",
                    details.verification_uri().to_string(),
                    details.user_code().secret().to_string()
                );
            },
        ));
        println!("Authenticated.");
    } else {
        identity_data = Some(IdentityData {
            display_name: "Zuri".into(),
            identity: Uuid::new_v4().to_string(),
            xuid: String::new(),
            title_id: None,
        });
    }

    let (send, recv) = channel::<Packet>(16);
    world.insert_non_send_resource(ClientWaiter {
        task: tokio::spawn(Client::connect(
            address.to_socket_addrs().unwrap().next().unwrap(),
            ClientData::default(),
            identity_data,
            live_token,
            PacketHandler {
                send_chan: send,
            },
        )),
    });
    world.insert_non_send_resource(recv);
}

/// Polls the connecter thread until the login has completed. When the login is complete, the
/// connection may be used by the game.
fn client_connection_system(world: &mut World) {
    let waiter = world.get_non_send_resource_mut::<ClientWaiter>();
    if waiter.is_none() {
        return;
    }
    let res = future::block_on(future::poll_once(&mut waiter.unwrap().into_inner().task));
    if res.is_none() {
        return;
    }
    match res.unwrap().unwrap() {
        Err(e) => {
            error!("Could not connect to the server: {e}");
            world.send_event(AppExit);
        }
        Ok((client, data)) => {
            let client = Arc::new(client);
            world.remove_non_send_resource::<ClientWaiter>();
            world.insert_non_send_resource(client.clone());
            world.insert_non_send_resource(data);
            info!("Connection has been completed");

            let (send, mut recv) = mpsc::channel::<Vec<Packet>>(1);
            world.insert_non_send_resource(send);
            tokio::spawn(async move {
                loop {
                    if let Some(pks) = recv.recv().await {
                        for mut pk in pks {
                            client.write_packet(&mut pk);
                        }
                    } else {
                        return;
                    }
                }
            });
        }
    }
}

/// Collects all the packets that should be sent and passes them on to the packet writer thread.
/// Should only run on the main thread.
fn send_packets(mut packets: ResMut<Events<Packet>>, chan: Option<NonSend<Sender<Vec<Packet>>>>) {
    if packets.is_empty() || chan.is_none() {
        return;
    }
    chan.unwrap().blocking_send(packets.drain().collect())
        .expect("Could not send packets to writer");
}

fn receive_packets(world: &mut World) {
    let mut opt_chan = world.get_non_send_resource_mut::<Receiver<Packet>>();
    if opt_chan.is_none() {
        return;
    }
    loop {
        match opt_chan.as_mut().unwrap().try_recv() {
            Err(err) => return match err {
                TryRecvError::Empty => {}
                TryRecvError::Disconnected => {
                    world.remove_non_send_resource::<Receiver<Vec<Packet>>>().unwrap();
                }
            },
            Ok(pk) => match pk {
                _ => {
                    warn!("Unhandled packet {pk}");
                }
            },
        };
    }
}

struct PacketHandler {
    send_chan: Sender<Packet>,
}

#[async_trait]
impl Handler for PacketHandler {
    async fn handle_incoming(&mut self, pk: Packet) -> Vec<Packet> {
        self.send_chan.send(pk).await.expect("TODO: panic message");
        vec![]
    }
}
