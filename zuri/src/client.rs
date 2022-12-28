use async_trait::async_trait;

use bevy::prelude::*;
use futures_lite::future;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::mpsc::error::TryRecvError;
use tokio::task::JoinHandle;
use uuid::Uuid;
use zuri_net::client::{Client, Handler};
use zuri_net::client::data::{ClientData, IdentityData};
use zuri_net::proto::packet::Packet;

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(init_client)
            .add_system(client_connection_system)
            .add_system(receive_packets);
    }
}

pub struct ClientWaiter {
    task: JoinHandle<Result<Client<PacketHandler>, String>>,
}

fn init_client(world: &mut World) {
    //let details = live::start_device_auth().unwrap();

    //println!(
    //    "Authenticate at {} using the code: {}",
    //    details.verification_uri().to_string(),
    //    details.user_code().secret().to_string()
    //);

    //let live_token = live::await_device_auth(details).unwrap();

    let (send, recv) = channel::<Vec<Packet>>(16);
    world.insert_non_send_resource(ClientWaiter {
        task: tokio::spawn(Client::connect(
            "127.0.0.1:19131".parse().unwrap(),
            ClientData::default(),
            Some(IdentityData {
                display_name: "Zuri".into(),
                identity: Uuid::new_v4().to_string(),
                title_id: None,
                xuid: "".into(),
            }),
            None,
            //None,
            //Some(live_token),
            PacketHandler {
                send_chan: send,
            },
        )),
    });
    world.insert_non_send_resource(recv);
}

fn client_connection_system(world: &mut World) {
    if let Some(waiter) = world.get_non_send_resource_mut::<ClientWaiter>() {
        if let Some(client) = future::block_on(future::poll_once(&mut waiter.into_inner().task)) {
            world.remove_non_send_resource::<ClientWaiter>();
            world.insert_non_send_resource(client.unwrap());
            info!("Connection has been completed");
        }
    }
}

fn receive_packets(world: &mut World) {
    if let Some(mut chan) = world.get_non_send_resource_mut::<Receiver<Vec<Packet>>>() {
        match chan.try_recv() {
            Err(err) => {
                match err {
                    TryRecvError::Empty => {},
                    TryRecvError::Disconnected => {
                        world.remove_non_send_resource::<Receiver<Vec<Packet>>>().unwrap();
                    },
                };
                return;
            }
            Ok(vec) => {
                for pk in vec {
                    match pk {
                        _ => {
                            warn!("Unhandled packet {pk}");
                        },
                    }
                }
            }
        };
    }
}

struct PacketHandler {
    send_chan: Sender<Vec<Packet>>,
}

#[async_trait]
impl Handler for PacketHandler {
    async fn handle_incoming(&mut self, pk: Packet) -> Vec<Packet> {
        self.send_chan.send(vec![pk]).await.expect("TODO: panic message");
        vec![]
    }
}
