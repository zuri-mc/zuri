use std::net::SocketAddr;
use bevy::app::AppExit;

use bevy::prelude::*;
use bevy::tasks::{IoTaskPool, Task};
use futures_lite::future;
use tokio::task::JoinHandle;
use uuid::Uuid;

use crate::client::{Client, Handler};
use crate::client::data::{ClientData, IdentityData};
use crate::connection::ConnError;

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(init_client)
            .add_system(client_connection_system);
    }
}

pub struct ClientWaiter {
    task: JoinHandle<Result<Client<PacketHandler>, String>>,
}

pub struct ClientContainer {
    client: Client<PacketHandler>,
}

fn init_client(world: &mut World) {
    world.insert_non_send_resource(ClientWaiter {
        task: tokio::spawn(Client::connect(
            "127.0.0.1:19132".parse().unwrap(),
            ClientData::default(),
            IdentityData {
                display_name: "Zuri".into(),
                identity: Uuid::new_v4().to_string(),
                title_id: None,
                xuid: "".into(),
            },
            PacketHandler,
        )),
    });
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

struct PacketHandler;

impl Handler for PacketHandler {}
