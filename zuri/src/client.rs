use std::net::SocketAddr;
use async_trait::async_trait;
use bevy::app::AppExit;

use bevy::prelude::*;
use bevy::tasks::{IoTaskPool, Task};
use futures_lite::future;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::mpsc::error::TryRecvError;
use tokio::task::JoinHandle;
use uuid::Uuid;
use zuri_net::client::{Client, Handler};
use zuri_net::client::data::{ClientData, IdentityData};
use zuri_net::proto::packet::level_chunk::LevelChunk;
use zuri_net::proto::packet::Packet;

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<LevelChunk>()
            .add_startup_system(init_client)
            .add_system(client_connection_system)
            .add_system(receive_packets);
    }
}

pub struct ClientWaiter {
    task: JoinHandle<Result<Client<PacketHandler>, String>>,
}

pub struct ClientContainer {
    client: Client<PacketHandler>,
}

fn init_client(world: &mut World) {
    //let details = live::start_device_auth().unwrap();

    //println!(
    //    "Authenticate at {} using the code: {}",
    //    details.verification_uri().to_string(),
    //    details.user_code().secret().to_string()
    //);

    //let live_token = live::await_device_auth(details).unwrap();

    let (send, recv) = channel(4);
    world.insert_non_send_resource(ClientWaiter {
        task: tokio::spawn(Client::connect(
            "127.0.0.1:19132".parse().unwrap(),
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
    if world.get_non_send_resource_mut::<Receiver<Packet>>().is_none() {
        return;
    }
    loop {
        // Get the resource from the world each time, since otherwise we are borrowing the world as
        // mutable more than once at the same time.
        let pk_res = world.get_non_send_resource_mut::<Receiver<Packet>>()
            .unwrap().try_recv();
        match pk_res {
            Err(err) => {
                match err {
                    TryRecvError::Empty => {}
                    TryRecvError::Disconnected => {
                        world.remove_non_send_resource::<Receiver<Vec<Packet>>>().unwrap();
                    }
                };
                return;
            }
            Ok(pk) => match pk {
                Packet::LevelChunk(pk) => world.send_event(pk),
                Packet::Disconnect(pk) => {
                    dbg!(pk);
                }
                _ => {
                    warn!("Packet `{pk}` was discarded");
                }
            }
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
