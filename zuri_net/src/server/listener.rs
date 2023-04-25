use std::net::SocketAddr;

use rust_raknet::error::RaknetError;
use rust_raknet::RaknetListener;
use tokio::select;
use tokio::sync::mpsc;

use crate::connection::Connection;
use crate::server::{Edition, Motd};

/// Server listener that listens to incoming client connections on a certain [SocketAddr]. Incoming
/// connections can be handled using the [Listener::accept] method. Before connections are passed
/// along, minecraft's login sequence is performed.
pub struct Listener {
    /// A channel that notifies the acceptor routine when to stop accepting players and close the
    /// socket.
    close_channel: mpsc::Sender<()>,
    /// A channel used to send connections back from the listener routine to whatever called the
    /// [Listener::accept] method.
    conn_channel: mpsc::Receiver<Result<Connection, RaknetError>>,
}

impl Listener {
    /// Starts listening and accepting players on the given [SocketAddr]. A handler is passed to
    /// handle incoming connections.
    pub async fn listen(addr: &SocketAddr) -> Result<Listener, RaknetError> {
        let (sender, recv) = mpsc::channel(1);
        let (pk_sender, conn_recv) = mpsc::channel(1);

        let mut listener = RaknetListener::bind(addr).await?;
        listener
            .set_full_motd(Motd {
                edition: Edition::Bedrock,
                local_motd: "Zuri".to_string(),
                motd: "Zuri Server".to_string(),
                player_count: 0,
                max_player_count: 1234,
            }.serialize(234872937684, 19132)).unwrap(); // todo: get the actual server id
        listener.listen().await;

        tokio::spawn(async move {
            let mut recv = recv;
            let conn_sender = pk_sender;
            let mut listener = listener;
            loop {
                select! {
                     _ = recv.recv() => {
                        // The listener has been closed. Stop listening.
                        return;
                    },
                    res = listener.accept() => {
                        let res: Result<Connection, RaknetError> = res.map(|v| Connection::new(v));
                        // todo: login sequence
                        if let Err(conn) = conn_sender.send(res).await {
                            if let Ok(conn) = conn.0 {
                                _ = conn.close().await;
                            }
                            panic!("Could not send connection.");
                        }
                    },
                }
            }
        });

        Ok(Listener {
            close_channel: sender,
            conn_channel: conn_recv,
        })
    }

    /// Accepts a new incoming minecraft connection.
    pub async fn accept(&mut self) -> Option<Result<Connection, RaknetError>> {
        self.conn_channel.recv().await
    }

    /// Stops listening for and accepting new connections.
    pub async fn close(&self) {
        if self.close_channel.send(()).await.is_err() {
            panic!("The receiver was dropped.");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, SocketAddr};

    use tokio::{runtime, time};
    use tokio::time::sleep;

    use crate::server::listener::Listener;

    #[test]
    fn test_server() {
        // todo: remove temporary test
        runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on((|| async {
                let addr = SocketAddr::new(IpAddr::from([0, 0, 0, 0]), 19132);
                let mut l = Listener::listen(&addr).await.unwrap();
                while let Some(conn) = l.accept().await {
                    match conn {
                        Ok(conn) => panic!("ok: {}", conn.peer_addr()),
                        Err(err) => panic!("err: {:?}", err),
                    };
                }
                sleep(time::Duration::from_secs(100)).await;
            })());
    }
}
